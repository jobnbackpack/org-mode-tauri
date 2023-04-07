use std::{path::Path, vec};

use orgize::Org;
use serde_json::to_string;

use crate::{ORG_FILE_PATH, ORG_DIR_PATH, file_reader::{read_file,read_dir}, org::{OrgSection, OrgNode}};

#[tauri::command]
pub fn get_org_children() -> Vec<String> {
    match read_file(&Path::new(ORG_FILE_PATH)) {
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

#[tauri::command]
pub fn get_org_file() -> Vec<OrgSection<'static>> {
    match read_file(&Path::new(ORG_FILE_PATH)) {
        Ok(s) => {
            let org = Org::parse_string(s);
            let d = org.document();

            OrgSection::new_section_vec_from_document(d, org)
        },
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    }
}

#[tauri::command]
pub fn get_all_org_files() -> Vec<Vec<OrgNode<'static>>> {
    match read_dir(&Path::new(ORG_DIR_PATH)) {
        Ok(s) => {
            let mut result: Vec<Vec<OrgNode<'static>>> = Vec::new();
            for file_string in s {
                let org = Org::parse_string(file_string);
                let d = org.document();

                result.push(OrgNode::get_all_nodes_from_doc(d, org));
            }
            result
        },
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    }
}

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

