// The desktop shell. It owns the window and the event loop, and it knows nothing about how a
// frame is produced - that is `anime_compositor`, which this crate depends on and which does
// not depend back. Document 06 requires the arrow to point only this way.
//
// The window is empty on purpose in this slice. Getting the shell, its ~260 crates and their
// licence record through CI is one reviewable change; putting pixels in the window is the
// next one, over the custom URI transport D-36 settled on.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("the window could not be created");
}
