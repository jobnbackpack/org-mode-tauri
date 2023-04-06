use std::path::Path;

use orgize::Org;
use serde_json::to_string;

use crate::{ORG_PATH, file_reader::read_file};

#[tauri::command]
pub fn get_org_children() -> Vec<String> {
    match read_file(&Path::new(ORG_PATH)) {
        Ok(s) => {
            let org = Org::parse_string(s);
            let d = org.document();

            d.children(&org).map(|node| node.title(&org).raw.to_string()).collect()
        },
        Err(e) => {
            println!("Error: {:?}", e);
            vec![String::from("error")]
        }
    }
}

struct OrgSection {
    title: String,
    nodes: Vec<String>
}

impl OrgSection {
    fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            nodes: Vec::new()
        }
    }
}

// TODO: create Vector of OrgSections
// #[tauri::command]
// pub fn get_all_todos() -> Vec<OrgSection> {
//     match read_file(&Path::new(ORG_PATH)) {
//         Ok(s) => {
//             let org = Org::parse_string(s);
//             let d = org.document();
//
//             let top_level_headings = d.children(&org);
//
//             let result: Vec<OrgSection> = Vec::new();
//
//             for heading in top_level_headings {
//                 let org_section = OrgSection::new(&heading.title(&org).raw[..]);
//                 heading.children(&org);
//             }
//
//             result
//         },
//         Err(e) => {
//             println!("Error: {:?}", e);
//             vec![]
//         }
//     }
// }

#[tauri::command]
pub fn get_org_file_json() -> String {
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

