const COMMANDS: &[&str] = &["speak", "stop", "info", "voices", "set_voice", "set_rate", "set_pitch", "set_volume"];

fn main() {
	tauri_plugin::Builder::new(COMMANDS).build();
	let msvc = std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
	if let Ok(dlls) = std::env::var("DEP_PRISMER_DELAY_LOAD_DLLS") {
		for dll in dlls.split(';').filter(|dll| !dll.is_empty() && msvc) {
			println!("cargo:rustc-link-arg=/DELAYLOAD:{dll}");
		}
		if msvc {
			println!("cargo:rustc-link-arg=/IGNORE:4199");
		}
		println!("cargo:delay_load_dlls={dlls}");
	}
}
