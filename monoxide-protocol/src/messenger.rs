use crate::protocol::{Message, Protocol};
use color_eyre::eyre::Result;
use rustc_hash::FxHashMap;
use stardust_xr_schemas::flex::{
    deserialize, flexbuffers::DeserializationError, serialize, FlexSerializeError,
};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use thiserror::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        unix::{OwnedReadHalf, OwnedWriteHalf},
        UnixStream,
    },
    sync::{mpsc, oneshot},
};
use tracing::instrument;

/// Error for all messenger-related failures.
#[derive(Error, Debug)]
pub enum MessengerError {
    /// The MessageReceiver has been dropped with pending futures
    #[error("Receiver has been dropped")]
    ReceiverDropped,
    #[error("IO Error: {0}")]
    IOError(std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(FlexSerializeError),
    #[error("Deserialization error: {0}")]
    DeserializationError(DeserializationError),
}
impl From<std::io::Error> for MessengerError {
    fn from(e: std::io::Error) -> Self {
        MessengerError::IOError(e)
    }
}

type PendingFuture = oneshot::Sender<Protocol>;
type PendingFutureSender = mpsc::UnboundedSender<(u64, PendingFuture)>;
type PendingFutureReceiver = mpsc::UnboundedReceiver<(u64, PendingFuture)>;

/// Receiving half of the messenger.
pub struct MessageReceiver {
    read: OwnedReadHalf,
    pending_futures: FxHashMap<u64, PendingFuture>,
    pending_future_rx: PendingFutureReceiver,
}
impl MessageReceiver {
    fn new(
        read: OwnedReadHalf,
        pending_future_rx: PendingFutureReceiver,
        _send_handle: MessageSenderHandle,
    ) -> Self {
        MessageReceiver {
            read,
            pending_futures: Default::default(),
            pending_future_rx,
        }
    }
    /// Take all the pending futures in the queue from method calls and store them for when the other side sends a method return.
    pub fn update_pending_futures(&mut self) {
        while let Ok((id, future)) = self.pending_future_rx.try_recv() {
            let _ = self.pending_futures.insert(id, future);
        }
    }
    /// Await a message from the socket, parse it, and handle it.
    pub async fn dispatch(&mut self) -> Result<Message, MessengerError> {
        let mut message_length_buffer: [u8; 4] = [0; 4];
        self.read.read_exact(&mut message_length_buffer).await?;
        let message_length: u32 = u32::from_ne_bytes(message_length_buffer);

        let mut message_buffer: Vec<u8> = std::vec::from_elem(0_u8, message_length as usize);
        self.read.read_exact(message_buffer.as_mut_slice()).await?;

        self.update_pending_futures();

        let message: Message =
            deserialize(&message_buffer).map_err(|e| MessengerError::DeserializationError(e))?;

        if let Some(future) = self.pending_futures.remove(&message.id) {
            let _ = future.send(message.clone().message);
        }
        Ok(message)
    }
}

/// Sender half of the messenger
pub struct MessageSender {
    write: OwnedWriteHalf,
    handle: MessageSenderHandle,
    message_rx: mpsc::UnboundedReceiver<Message>,
}
impl MessageSender {
    fn new(write: OwnedWriteHalf, pending_future_tx: PendingFutureSender) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();
        let max_message_id = Arc::new(AtomicU64::new(0));
        MessageSender {
            write,
            handle: MessageSenderHandle {
                message_tx,
                pending_future_tx: pending_future_tx.clone(),
                max_message_id: max_message_id.clone(),
            },
            message_rx,
        }
    }
    /// Send all the queued messages from the handles
    pub async fn flush(&mut self) -> Result<(), MessengerError> {
        while let Some(message) = self.message_rx.recv().await {
            self.send(message).await?;
        }
        Ok(())
    }
    /// Send a message and await until sent.
    pub async fn send(&mut self, message: Message) -> Result<(), MessengerError> {
        let message = serialize(message).map_err(|e| MessengerError::SerializationError(e))?;
        let message_length = message.len() as u32;
        self.write.write_all(&message_length.to_ne_bytes()).await?;
        self.write.write_all(&message).await?;
        Ok(())
    }
    /// Get a handle to send messages from anywhere.
    pub fn handle(&self) -> MessageSenderHandle {
        self.handle.clone()
    }
}

/// Handle to the message sender, so you can synchronously send messages from anywhere without blocking.
#[derive(Clone)]
pub struct MessageSenderHandle {
    message_tx: mpsc::UnboundedSender<Message>,
    pending_future_tx: PendingFutureSender,
    max_message_id: Arc<AtomicU64>,
}
impl MessageSenderHandle {
    #[instrument(level = "trace", skip_all)]
    fn send(&self, message: Message) -> Result<(), MessengerError> {
        self.message_tx
            .send(message)
            .map_err(|_| MessengerError::ReceiverDropped)
    }
    /// Call a method immediately, await until other side sends back a response or the message fails to send.
    pub async fn method(&mut self, message: Protocol) -> Result<Protocol, MessengerError> {
        let (tx, rx) = oneshot::channel();
        let id = self.max_message_id.load(Ordering::Relaxed);
        self.pending_future_tx
            .send((id, tx))
            .map_err(|_| MessengerError::ReceiverDropped)?;
        self.send(Message { id, message })?;
        self.max_message_id.store(id + 1, Ordering::Relaxed);
        rx.await.map_err(|_| MessengerError::ReceiverDropped)
    }
}

/// Create 2 messenger halves from a connection to a stardust client or server.
pub fn create(connection: UnixStream) -> (MessageSender, MessageReceiver) {
    let (read, write) = connection.into_split();
    let (pending_future_tx, pending_future_rx) = mpsc::unbounded_channel();
    let sender = MessageSender::new(write, pending_future_tx);
    let receiver = MessageReceiver::new(read, pending_future_rx, sender.handle());
    (sender, receiver)
}
