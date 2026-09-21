use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};

use prismer::{Backend, Features as Supports, Prism};

use crate::types::{respond_unavailable, Features, Info, Message, Outcome, Reply, Voice};

const STOPPED: &str = "speech worker stopped";

pub struct Handle {
	tx: Sender<Message>,
}

impl Handle {
	pub fn send(&self, message: Message) -> Result<(), String> {
		self.tx.send(message).map_err(|_| STOPPED.to_string())
	}

	pub fn ask<T>(&self, make: impl FnOnce(Reply<T>) -> Message) -> Result<T, String> {
		let (reply, rx) = channel();
		self.send(make(reply))?;
		rx.recv().map_err(|_| STOPPED.to_string())
	}
}

pub fn spawn() -> Handle {
	start().0
}

fn start() -> (Handle, JoinHandle<()>) {
	let (tx, rx) = channel();
	let thread = thread::spawn(move || run(rx));
	(Handle { tx }, thread)
}

fn run(rx: Receiver<Message>) {
	let prism = Prism::new().ok();
	let backend = prism.as_ref().and_then(|p| p.create_best().ok());
	for message in rx {
		match backend.as_ref() {
			Some(backend) => handle(backend, message),
			None => respond_unavailable(message),
		}
	}
}

fn handle(backend: &Backend<'_>, message: Message) {
	match message {
		Message::Speak { text, interrupt } => {
			let result = if backend.supports(Supports::OUTPUT) {
				backend.output(&text, interrupt)
			} else {
				backend.speak(&text, interrupt)
			};
			if let Err(err) = result {
				log::warn!("prism speak failed: {err}");
			}
		}
		Message::Stop => {
			if let Err(err) = backend.stop() {
				log::warn!("prism stop failed: {err}");
			}
		}
		Message::Info { reply } => drop(reply.send(info(backend))),
		Message::Voices { reply } => drop(reply.send(voices(backend))),
		Message::SetVoice { id, reply } => drop(reply.send(apply(backend.set_voice(id)))),
		Message::SetRate { value, reply } => drop(reply.send(apply(backend.set_rate(value)))),
		Message::SetPitch { value, reply } => drop(reply.send(apply(backend.set_pitch(value)))),
		Message::SetVolume { value, reply } => drop(reply.send(apply(backend.set_volume(value)))),
	}
}

fn apply(result: prismer::Result<()>) -> Outcome {
	result.map_err(|err| err.to_string())
}

fn info(backend: &Backend<'_>) -> Info {
	Info {
		available: true,
		backend: Some(backend.name()),
		features: Features {
			voice: backend.supports(Supports::SET_VOICE),
			rate: backend.supports(Supports::SET_RATE),
			pitch: backend.supports(Supports::SET_PITCH),
			volume: backend.supports(Supports::SET_VOLUME),
		},
	}
}

fn voices(backend: &Backend<'_>) -> Vec<Voice> {
	if backend.supports(Supports::REFRESH_VOICES) {
		let _ = backend.refresh_voices();
	}
	backend
		.voices()
		.map(|list| list.into_iter().map(|v| Voice { id: v.id, name: v.name, language: v.language }).collect())
		.unwrap_or_default()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::types::Message;

	#[test]
	fn info_answers_and_the_thread_exits_when_the_handle_drops() {
		let (handle, thread) = start();
		let info = handle.ask(|reply| Message::Info { reply }).unwrap();
		assert_eq!(info.available, info.backend.is_some());
		if info.available {
			handle.send(Message::Speak { text: "test".into(), interrupt: true }).unwrap();
			handle.send(Message::Stop).unwrap();
		}
		drop(handle);
		thread.join().unwrap();
	}
}
