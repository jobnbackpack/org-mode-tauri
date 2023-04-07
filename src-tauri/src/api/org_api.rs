use std::{path::Path, vec};

use orgize::{Org, elements::{Planning}};
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
    level: usize,
    planning: Option<Box<Planning<'static>>>
}

impl OrgNode {
    fn new(name: &str, state: OrgTodoState, level: usize, planning: Option<Box<Planning<'static>>>) -> Self {
        // let org_planner: Option<OrgPlanner>;
        // match planning {
        //     Some(p) => {
        //         org_planner = Some(OrgPlanner::new(p.deadline.clone(), p.scheduled.clone(), p.closed.clone()));
        //     },
        //     None => org_planner = None
        // }
        Self {
            name: String::from(name),
            state,
            level,
            planning
        }
    }
}

// #[derive(Debug, serde::Serialize)]
// pub struct OrgPlanner {
//     deadline: Option<Timestamp>,
//     scheduled: Option<Timestamp<'static>>,
//     closed: Option<Timestamp<'static>>
// }
//
// impl OrgPlanner {
//     fn new(deadline: Option<Timestamp>, scheduled: Option<Timestamp>, closed: Option<Timestamp>) -> Self {
//         match deadline {
//             Some(d) => d.
//             
//         }
//         Self {
//             deadline: deadline.clone(),
//             scheduled: scheduled.clone(),
//             closed: closed.clone()
//         }
//     }
// }
// let start = orgize::elements::Datetime {
//                 year: 2109,
//                 month: 11,
//                 day: 11,
//                 dayname: "Mon".into(),
//                 hour: None,
//                 minute: None,
//             };
//             let scheduled = orgize::elements::Timestamp::Active {
//                 start,
//                 repeater: None,
//                 delay: None,
//             };
//             title.planning = Some(Box::new(orgize::elements::Planning {
//                 deadline: None,
//                 ne20)
//
//                 scheduled: Some(scheduled),
//                 closed: None,
//             }));

#[tauri::command]
pub fn get_org_file() -> Vec<OrgSection> {
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
                            node.level(),
                            node.title(&org).planning.clone()
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

