#[cfg(all(feature = "static", feature = "shared"))]
compile_error!("enable either the static or the shared feature, not both");

mod commands;
mod types;

#[cfg(not(target_os = "android"))]
mod worker;
#[cfg(target_os = "android")]
#[path = "stub.rs"]
mod worker;

pub use types::{Features, Info, Voice};

use tauri::{
	plugin::{Builder, TauriPlugin},
	Manager, Runtime,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
	Builder::new("prism")
		.invoke_handler(tauri::generate_handler![
			commands::speak,
			commands::stop,
			commands::info,
			commands::voices,
			commands::set_voice,
			commands::set_rate,
			commands::set_pitch,
			commands::set_volume,
		])
		.setup(|app, _api| {
			app.manage(worker::spawn());
			Ok(())
		})
		.build()
}
