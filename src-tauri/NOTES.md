# src-tauri (reconstruction notes)

The binary was built with **Tauri v2.12.1** (not v1).

To rebuild the app shell:

1. `npm create tauri-app` (or `cargo install tauri-cli --version 2.12.1`)
2. Point the frontend `dist` to your UI and set the dev/build config in
   `tauri.conf.json`.
3. Register the commands from `src/commands/mod.rs` in
   `tauri::Builder::default().invoke_handler(tauri::generate_handler![...])`.

**IMPORTANT — the frontend was NOT found in the binary.** No HTML/JS/CSS
of the app's UI is embedded in the exe (only test-fixture HTML from
third-party crates). If your UI was a separate `dist/` folder, that part
cannot be recovered from this exe — you'll need to rewrite it. The good
news: the full backend API it talked to (16 commands + 5 events) is
documented in `src/commands/mod.rs`, so you know exactly what the new
UI must call.

Recovered Tauri events (backend -> frontend):
- `workshop://queue-updated`
- `workshop://queue-started`
- `workshop://queue-stopped`
- `workshop://item-completed`
- `workshop://progress`
