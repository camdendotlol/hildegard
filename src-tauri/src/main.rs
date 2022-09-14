#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use self::models::*;
use diesel::prelude::*;
use hildegard::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn count() -> String {
    use self::schema::scripts::dsl::*;
    let connection = &mut establish_connection();

    let mut script_vec = Vec::new();

    match scripts.load::<Script>(connection) {
        Ok(mut list) => script_vec.append(&mut list),
        _ => (),
    }

    format!("{}", script_vec.len())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
