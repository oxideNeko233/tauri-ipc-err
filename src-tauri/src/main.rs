// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SomeJson {
    info: String,
}

#[tauri::command]
async fn err() -> Result<SomeJson, SomeJson> {
    Err(SomeJson {
        info: "info".to_string(),
    })
}

#[tauri::command]
async fn ok() -> Result<SomeJson, SomeJson> {
    Ok(SomeJson {
        info: "info".to_string(),
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ok, err])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
