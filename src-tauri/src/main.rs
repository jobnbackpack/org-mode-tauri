// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use orgize::Org;
use serde_json::to_string;
mod file_reader;
use file_reader::read_file;

#[tauri::command]
fn get_org_children() -> Vec<String> {
    match read_file(&Path::new("C:/Users/Rjanzen/Dropbox/org/agenda.org")) {
        Ok(s) => {
            let org = Org::parse(&s[..]);

            let d = org.document();

            d.children(&org).map(|node| node.title(&org).raw.to_string()).collect()
        },
        Err(e) => {
            println!("Error: {:?}", e);
            vec![String::from("error")]
        }
    }
}

#[tauri::command]
fn get_org_file_json() -> String {
    let org = Org::parse(
        r#"
* TODO this is a simple todo
* TODO asoeutnho
* satoehuosh
* h1
** h1_1
** h1_2
        "#,
        );

    to_string(&org).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_org_children, get_org_file_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
