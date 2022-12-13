#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;

#[tauri::command]
fn get_pic_binary(path: &str) -> Vec<u8> {
    let pic_bin = fs::read(path).expect("Failed to read file");
    pic_bin
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_pic_binary])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
