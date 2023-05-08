use crate::prelude::*;
use monoxide_protocol::messenger::*;
use openxr_sys::{Instance, InstanceCreateInfo};
use dirs::runtime_dir;
use tokio::{runtime::Runtime, net::UnixStream};

#[handle(Instance)]
pub struct XrInstance {
    pub extensions: Vec<String>,
    pub runtime: Runtime,
    pub handle: MessageSenderHandle,
}

impl XrInstance {
    pub fn new(info: InstanceCreateInfo) -> Result<Self, XrResult> {
        let mut extensions = vec![];
        for str in str_slice_from_const_arr(info.enabled_extension_names, info.enabled_extension_count as usize) {
            let str = str_from_const_char(*str)?;
            extensions.push(String::from(str));
        }
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .map_err(|_| XrResult::ERROR_RUNTIME_UNAVAILABLE)?;
        let (mut sender, mut reciever) = 
            runtime.block_on(async {
                let socket_path = runtime_dir()
		            .ok_or_else(|| XrResult::ERROR_RUNTIME_UNAVAILABLE)?
		            .join("monoxide");
                match UnixStream::connect(socket_path).await {
                    Ok(stream) => Ok(create(stream)),
                    Err(_) => Err(XrResult::ERROR_RUNTIME_UNAVAILABLE),
                }
            })?;
        let handle = sender.handle();
        runtime.spawn(
			async move { while reciever.dispatch().await.is_ok() {} },
		);
        runtime.spawn(async move {
            sender.flush().await
        });
        Ok(Self {
            extensions,
            runtime,
            handle,
        })
    }
}