use std::{path::Path, vec};

use crate::{ORG_DIR_PATH, file_reader::read_dir, org::{OrgNode, OrgFile}};

#[tauri::command]
pub fn get_all() -> Vec<OrgFile>  {
    match read_dir(&Path::new(ORG_DIR_PATH)) {
        Ok(raw_files) => {
            let mut result = Vec::new();
            for raw_file in raw_files {

                result.push(OrgFile {
                    name: raw_file.name,
                    path: raw_file.path,
                    nodes: OrgNode::get_all_children(String::from(raw_file.raw))
                }
                );
            }
            result
        },
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    }
}

