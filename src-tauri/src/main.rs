// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod reader;
use std::collections::BTreeMap;

#[tauri::command(rename_all = "snake_case")]
fn read_temps() -> BTreeMap<String, usize> {
    reader::get_devices()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_temps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
