#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const AUDIO: &[u8] = include_bytes!("../../data/output.mp3");

#[tauri::command]
fn audio() -> Result<Vec<u8>, String> {
    Ok(AUDIO.to_vec())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
