#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use hildegard::{scripts::management::count, __cmd__count};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
