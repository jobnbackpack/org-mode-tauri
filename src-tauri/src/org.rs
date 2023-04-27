use orgize::{Document, Org};
use starsector::*;

#[derive(Debug, serde::Serialize)]
pub struct OrgFile {
    pub name: String,
    pub path: String,
    pub nodes: Vec<OrgNode>
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
    priority: Option<char>,
    planning: String, // TODO: needs a better type
    nodes: Option<Vec<OrgNode>>
}

impl OrgNode {
    pub fn get_all_children(raw: String) -> Vec<OrgNode> {
        let mut arena = Arena::default();
        let doc = arena.parse_string(raw);
        //
        // println!("Children: {:?}", children);
        //
        // let headline = children[1].headline(&arena, None);
        // let headline = headline.unwrap();
        //
        // let sub_children: Vec<_> = children[0].children(&arena).collect();
        // let sub_headline = sub_children[0].headline(&arena, None).unwrap();
        //
        // println!("Headline: {:?}", headline);
        // println!("Sub Headline: {:?}", sub_headline.planning());
        let top_level_children: Vec<Section> = doc.root.children(&arena).collect();
        println!("Trying to print top level children: {:?}", top_level_children);

        let mut result: Vec<OrgNode> = Vec::new();

        // INFO: Top Level
        for child in top_level_children {
            let headline = child.headline(&arena, None).unwrap();
            let state = headline.keyword();
            let planning = headline.planning();
            println!("Headline: {:?}", headline);
            println!("State: {:?}", state);
            println!("Planning: {:?}", planning);

            // INFO: Sub Level
            let sub_level_children: Vec<Section> = child.children(&arena).collect();
            for sub_child in sub_level_children {
                let sub_headline = sub_child.headline(&arena, None).unwrap();
                let sub_state = sub_headline.keyword();
                let sub_planning = headline.planning();
                println!("Subheadline Headline: {:?}", sub_headline);
                println!("Subheadline State: {:?}", sub_state);
                println!("Subheadline Planning: {:?}", sub_planning);

            }
            // {
            //     Some("TODO") => OrgTodoState::TODO,
            //     Some("DONE") => OrgTodoState::DONE,
            //     None => OrgTodoState::NONE,
            //     _ => OrgTodoState::NONE
            // };
            // let node_title = &child.title(&org).clone();
            // let title = node_title.raw.to_string();
            // let planning = &node_title.planning;
            // let priority = &node_title.priority;
            // let mut sub_nodes: Vec<OrgNode> = Vec::new();
            //
            // for sub_node in child.children(&org) {
            //     let state = match sub_node.title(&org).keyword.as_deref() {
            //         Some("TODO") => OrgTodoState::TODO,
            //         Some("DONE") => OrgTodoState::DONE,
            //         None => OrgTodoState::NONE,
            //         _ => OrgTodoState::NONE
            //     };
            //     let node_title = &sub_node.title(&org).clone();
            //     let title = node_title.raw.to_string();
            //     let planning = &node_title.planning;
            //     let priority = &node_title.priority;
            //     let mut sub_sub_nodes: Vec<OrgNode> = Vec::new();
            //
            //     for sub_sub_node in sub_node.children(&org) {
            //         let state = match sub_sub_node.title(&org).keyword.as_deref() {
            //             Some("TODO") => OrgTodoState::TODO,
            //             Some("DONE") => OrgTodoState::DONE,
            //             None => OrgTodoState::NONE,
            //             _ => OrgTodoState::NONE
            //         };
            //         let node_title = &sub_sub_node.title(&org).clone();
            //         let title = node_title.raw.to_string();
            //         let planning = &node_title.planning;
            //         let priority = &node_title.priority;
            //
            //         sub_sub_nodes.push(
            //             OrgNode {
            //                 name: title,
            //                 state,
            //                 level: sub_sub_node.level(),
            //                 planning: planning.clone(),
            //                 priority: *priority,
            //                 nodes: None
            //             }
            //             );
            //     }
            //     sub_nodes.push(
            //         OrgNode {
            //             name: title,
            //             state,
            //             level: sub_node.level(),
            //             planning: planning.clone(),
            //             priority: *priority,
            //             nodes: Some(sub_sub_nodes)
            //         }
            //         );
            // }

            // result.push(
            //     OrgNode {
            //         name: title,
            //         state,
            //         level: child.level(),
            //         planning: planning.clone(),
            //         priority: *priority,
            //         nodes: Some(sub_nodes)
            //     }
            //     );
        }
        result
    }
}
