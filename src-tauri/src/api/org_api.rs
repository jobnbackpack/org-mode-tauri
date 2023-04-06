use std::{path::Path, vec, any::Any, borrow::Borrow};

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

#[derive(Debug, serde::Serialize)]
pub struct OrgSection {
    pub title: String,
    pub nodes: Vec<OrgNode>
}

impl OrgSection {
    fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            nodes: Vec::new()
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub enum OrgTodoState {
    TODO,
    DONE,
    NONE
}

#[derive(Debug, serde::Serialize)]
pub struct OrgNode {
    name: String,
    state: OrgTodoState,
    level: usize
}

impl OrgNode {
    fn new(name: &str, state: OrgTodoState, level: usize) -> Self {
        Self {
            name: String::from(name),
            state,
            level
        }
    }
}

#[tauri::command]
pub fn get_all_todos() -> Vec<OrgSection> {
    match read_file(&Path::new(ORG_PATH)) {
        Ok(s) => {
            let org = Org::parse_string(s);
            let d = org.document();

            let top_level_headings = d.children(&org);

            let mut result: Vec<OrgSection> = Vec::new();

            for heading in top_level_headings {
                let mut org_section = OrgSection::new(&heading.title(&org).raw[..]);
                let sub_headings = heading.children(&org);
                for node in sub_headings {
                    println!("Debug: {:?}", node.headline_node());
                    // println!("Debug: {:?}", node.children(&org).map(|child| child.title(&org).raw.to_string()).collect::<String>());
                    let state = match node.title(&org).keyword.as_deref() {
                        Some("TODO") => OrgTodoState::TODO,
                        Some("DONE") => OrgTodoState::DONE,
                        None => OrgTodoState::NONE,
                        _ => OrgTodoState::NONE
                    };
                    org_section.nodes.push(
                        OrgNode::new(
                            &node.title(&org).raw[..],
                            state,
                            node.level()
                        )
                    );
                }
                result.push(org_section);
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

