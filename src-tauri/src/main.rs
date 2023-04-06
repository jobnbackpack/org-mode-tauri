// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_reader;
mod api;

use api::org_api;

// TODO: make this configurable
const ORG_PATH: &str = "C:/Users/Rjanzen/Dropbox/org/agenda.org";


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            org_api::get_all_todos,
            org_api::get_org_children,
            org_api::get_org_file_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
