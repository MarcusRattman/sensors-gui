// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{
    collections::HashMap,
    process::{Command, Stdio},
};

#[tauri::command(rename_all = "snake_case")]
fn read_temps<'a>() -> HashMap<String, String> {
    let sensors = Command::new("sensors")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let grep = Command::new("grep")
        .arg("-A 0")
        .arg("°C")
        .stdin(Stdio::from(sensors.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap()
        .stdout;

    let output = String::from_utf8(grep)
        .unwrap()
        .replace(":", "")
        .replace("--", "")
        .replace("=", "");

    let output = output.split_whitespace().collect::<Vec<_>>();

    output
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .fold(HashMap::new(), |mut acc, (x, y)| {
            acc.insert(x.to_string(), y.to_string());
            acc
        })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_temps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
