# Tauri Plugin Prism

`tauri-plugin-prism` gives Tauri applications native screen reader and speech output through [Prism](https://github.com/ethindp/prism).

When an application needs to send timely text to assistive technologies, ARIA live regions and the Web Speech API are only somewhat effective. Live regions depend on browser and screen reader behavior, while the Web Speech API speaks through a browser voice rather than the active screen reader.

Tauri lets a web frontend call native Rust code and package it as a desktop application. This plugin uses that native bridge to select an available Prism backend and send text directly to a screen reader or text to speech engine. Speech runs on a dedicated worker thread so it does not block the webview.

This plugin was built to complement [audiogame-utils](https://github.com/cartertemm/audiogame-utils), which supplies everything one might need to build accessible games natively and in the browser, powered by the same codebase.

## Prerequisites

You need:

* A Tauri 2 application
* A Rust toolchain
* Node.js and npm, or another JavaScript package manager
* CMake 3.24 or newer
* A compiler with C++23 support

The desktop requirements inherited from Prism are:

* Windows 10 or newer
* macOS 11 or newer
* Linux with the GLib 2.68 ABI or newer. Direct Orca output generally requires GLib and glibmm 2.80 or newer and Orca 49 or newer. Speech Dispatcher output requires ABI version 2, available in Speech Dispatcher 0.11.1 and newer.

Android and iOS builds compile, but the plugin currently uses an unavailable stub on those platforms and does not produce speech.

## Installation

The Rust crate and JavaScript package are not currently published to a registry. Clone and build the plugin beside your Tauri application:

```sh
git clone https://github.com/cartertemm/tauri-plugin-prism.git
cd tauri-plugin-prism
npm ci
npm run build
```

From your application directory, install the JavaScript package from that checkout:

```sh
npm install ../tauri-plugin-prism
```

Add the Rust crate to `src-tauri/Cargo.toml`. This example assumes the application and plugin directories have the same parent:

```toml
[dependencies]
tauri-plugin-prism = { path = "../../tauri-plugin-prism" }
```

You can use the Git repository for the Rust dependency instead:

```toml
[dependencies]
tauri-plugin-prism = { git = "https://github.com/cartertemm/tauri-plugin-prism" }
```

## Setting up a basic application

Register the plugin with the Tauri builder in `src-tauri/src/lib.rs`:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_prism::init())
		.run(tauri::generate_context!())
		.expect("error while running Tauri application");
}
```

Add `prism:default` to the permissions array in your Tauri capability file, usually `src-tauri/capabilities/default.json`:

```json
{
	"permissions": [
		"core:default",
		"prism:default"
	]
}
```

The default permission allows every command exposed by the plugin.

Check that a backend is available before speaking:

```ts
import { info, speak } from 'tauri-plugin-prism-api'

const speech = await info()

if (speech.available) {
	await speak('Welcome to the application.', true)
} else {
	console.warn('No screen reader or speech backend is available.')
}
```

The second argument to `speak` controls interruption. Passing `true` asks the backend to stop its current output before speaking the new text.

## API reference

All functions return promises and invoke the native plugin.

### `speak(text: string, interrupt = false): Promise<void>`

Sends text to the selected backend. When the backend supports combined screen reader output, the plugin uses it. Otherwise, it uses speech output. Set `interrupt` to `true` when the new text should replace current output.

### `stop(): Promise<void>`

Stops the current output when supported by the selected backend.

### `info(): Promise<Info>`

Returns backend availability, the selected backend name, and the controls it supports. Call this before using optional controls.

### `voices(): Promise<Voice[]>`

Returns the voices reported by the selected backend. The list is empty when no backend is available or voice enumeration is unsupported.

### `setVoice(id: number): Promise<void>`

Selects a voice by the identifier returned from `voices()`. The promise rejects if voice selection is unavailable or the identifier is invalid.

### `setRate(value: number): Promise<void>`

Sets the speech rate. Valid values are defined by the selected backend. Check `info().features.rate` before calling it.

### `setPitch(value: number): Promise<void>`

Sets the speech pitch. Valid values are defined by the selected backend. Check `info().features.pitch` before calling it.

### `setVolume(value: number): Promise<void>`

Sets the speech volume. Valid values are defined by the selected backend. Check `info().features.volume` before calling it.

### Types

```ts
interface Features {
	voice: boolean
	rate: boolean
	pitch: boolean
	volume: boolean
}

interface Info {
	available: boolean
	backend: string | null
	features: Features
}

interface Voice {
	id: number
	name: string
	language: string | null
}
```

When `Info.available` is `false`, `backend` is `null`, every feature is `false`, and `voices()` returns an empty array.

## Cargo features

The default `static` feature builds and links Prism statically:

```toml
tauri-plugin-prism = { path = "../../tauri-plugin-prism" }
```

To link Prism as a shared library, disable the default features and enable `shared`:

```toml
tauri-plugin-prism = { path = "../../tauri-plugin-prism", default-features = false, features = ["shared"] }
```

With shared linking, the Prism library must be available to the operating system loader at runtime. Do not enable `static` and `shared` together.

## Current limitations

Prism selects the best available backend once when the plugin starts. The current API cannot choose another backend or refresh that selection while the application is running.

Android and iOS use an unavailable stub. On those platforms, `info()` reports that no backend is available and `voices()` returns an empty array.

The plugin exposes stop control, but not Prism's pause and resume controls.

## TODO

1. Add native Android and iOS speech support.
2. Add runtime backend selection or refresh.
3. Add pause and resume controls.
