use std::{path::PathBuf, fs::{File, self}};

use cluFlock::ExclusiveFlock;
use dirs::runtime_dir;
use tokio::net::UnixListener;


pub async fn main() {
    let path = get_free_socket_path().unwrap();
    println!("{}", path.to_str().unwrap());
    let listener = UnixListener::bind(path).unwrap();
    loop {
        match listener.accept().await {
            Ok((_stream, _addr)) => {
                println!("new client!");
            }
            Err(_e) => { /* connection failed */ }
        }
    }
}

/// Get the lowest numbered socket path not taken by another server, if available.
pub fn get_free_socket_path() -> Option<PathBuf> {
	// Get the base XDG directories
	let runtime_dir = runtime_dir()?;
    let socket_path = runtime_dir.join("monoxide");
    let socket_lock_path = runtime_dir.join(format!("monoxide.lock"));
    match File::create(socket_lock_path) {
    	Err(_) => None,
    	Ok(file) => match file.try_lock() {
    		Err(_) => None,
    		Ok(_) => match fs::remove_file(socket_path.clone()) {
    			Ok(_) => return Some(socket_path),
    			Err(err) => match err.kind() {
    				std::io::ErrorKind::NotFound => return Some(socket_path),
    				_ => None,
    			},
    		},
    	},
    }
}