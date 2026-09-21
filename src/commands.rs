use tauri::{command, State};

use crate::types::{Info, Message, Voice};
use crate::worker::Handle;

#[command]
pub fn speak(handle: State<'_, Handle>, text: String, interrupt: bool) -> Result<(), String> {
	handle.send(Message::Speak { text, interrupt })
}

#[command]
pub fn stop(handle: State<'_, Handle>) -> Result<(), String> {
	handle.send(Message::Stop)
}

#[command]
pub fn info(handle: State<'_, Handle>) -> Result<Info, String> {
	handle.ask(|reply| Message::Info { reply })
}

#[command]
pub fn voices(handle: State<'_, Handle>) -> Result<Vec<Voice>, String> {
	handle.ask(|reply| Message::Voices { reply })
}

#[command]
pub fn set_voice(handle: State<'_, Handle>, id: usize) -> Result<(), String> {
	handle.ask(|reply| Message::SetVoice { id, reply })?
}

#[command]
pub fn set_rate(handle: State<'_, Handle>, value: f32) -> Result<(), String> {
	handle.ask(|reply| Message::SetRate { value, reply })?
}

#[command]
pub fn set_pitch(handle: State<'_, Handle>, value: f32) -> Result<(), String> {
	handle.ask(|reply| Message::SetPitch { value, reply })?
}

#[command]
pub fn set_volume(handle: State<'_, Handle>, value: f32) -> Result<(), String> {
	handle.ask(|reply| Message::SetVolume { value, reply })?
}
