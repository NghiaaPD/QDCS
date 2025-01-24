#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod services;

fn main() {
    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![test_docx])
        // .invoke_handler(tauri::generate_handler![greet, chui])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
