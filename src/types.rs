use std::sync::mpsc::Sender;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Features {
	pub voice: bool,
	pub rate: bool,
	pub pitch: bool,
	pub volume: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Info {
	pub available: bool,
	pub backend: Option<String>,
	pub features: Features,
}

#[derive(Clone, Debug, Serialize)]
pub struct Voice {
	pub id: usize,
	pub name: String,
	pub language: Option<String>,
}

pub type Reply<T> = Sender<T>;
pub type Outcome = Result<(), String>;

pub enum Message {
	Speak { text: String, interrupt: bool },
	Stop,
	Info { reply: Reply<Info> },
	Voices { reply: Reply<Vec<Voice>> },
	SetVoice { id: usize, reply: Reply<Outcome> },
	SetRate { value: f32, reply: Reply<Outcome> },
	SetPitch { value: f32, reply: Reply<Outcome> },
	SetVolume { value: f32, reply: Reply<Outcome> },
}

pub const NO_BACKEND: &str = "no speech backend";

pub fn unavailable() -> Info {
	Info {
		available: false,
		backend: None,
		features: Features { voice: false, rate: false, pitch: false, volume: false },
	}
}

pub fn respond_unavailable(message: Message) {
	match message {
		Message::Speak { .. } | Message::Stop => {}
		Message::Info { reply } => drop(reply.send(unavailable())),
		Message::Voices { reply } => drop(reply.send(Vec::new())),
		Message::SetVoice { reply, .. }
		| Message::SetRate { reply, .. }
		| Message::SetPitch { reply, .. }
		| Message::SetVolume { reply, .. } => drop(reply.send(Err(NO_BACKEND.to_string()))),
	}
}
