#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod functions;
mod services;
mod middleware;

use middleware::fill_format::read_docx_content;
use services::querydb::query_db;
use std::path::PathBuf;
use serde_json;

#[tauri::command]
async fn process_docx(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(file_path);

    let docx_result = read_docx_content(path.to_str().unwrap_or(""));
    
    let db_result = query_db();
    
    match (docx_result, db_result) {
        (Ok(questions), Ok(embeddings)) => {
            let result = serde_json::json!({
                "questions": questions,
                "embeddings": embeddings
            });
            Ok(result.to_string())
        },
        (Err(e), _) => Err(e.to_string()),
        (_, Err(e)) => Err(e.to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_docx])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
