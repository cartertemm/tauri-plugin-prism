use std::sync::mpsc::channel;

use crate::types::{respond_unavailable, Message, Reply, NO_BACKEND};

pub struct Handle;

impl Handle {
	pub fn send(&self, message: Message) -> Result<(), String> {
		respond_unavailable(message);
		Ok(())
	}

	pub fn ask<T>(&self, make: impl FnOnce(Reply<T>) -> Message) -> Result<T, String> {
		let (reply, rx) = channel();
		respond_unavailable(make(reply));
		rx.recv().map_err(|_| NO_BACKEND.to_string())
	}
}

pub fn spawn() -> Handle {
	Handle
}
