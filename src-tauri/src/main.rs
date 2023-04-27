// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_reader;
mod api;
mod org;

use api::org_api;

// TODO: make this configurable
const ORG_DIR_PATH: &str = "/home/rjanzen/test-data/";

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            org_api::get_all,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

