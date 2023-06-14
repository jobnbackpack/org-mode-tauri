use starsector::*;

#[derive(Debug, serde::Serialize)]
pub struct OrgFile {
    pub name: String,
    pub path: String,
    pub nodes: Vec<OrgNode>
}

#[derive(Debug, serde::Serialize)]
pub struct OrgPlanning<'a> {
    pub deadline: Option<Timestamp<'a>>,
    pub scheduled: Option<Timestamp<'a>>,
    pub closed: Option<Timestamp<'a>>,
}

#[derive(Debug, serde::Serialize)]
pub struct OrgNode {
    name: String,
    state: String,
    level: usize,
    priority: Option<char>,
    planning: Option<OrgPlanning<'static>>, 
    nodes: Option<Vec<OrgNode>>
}

impl OrgNode {
    pub fn get_all_children(raw: String) -> Vec<OrgNode> {
        let mut arena = Arena::default();
        let doc = arena.parse_string(raw);

        let top_level_children: Vec<Section> = doc.root.children(&arena).collect();

        let mut result: Vec<OrgNode> = Vec::new();

        // INFO: Top Level
        for child in top_level_children {
            let headline = child.headline(&arena, None).unwrap();
            let state = match headline.keyword() {
                Some(state) => state.into(),
                None => String::from("NONE")
            };
            let planning = headline.planning();

            // INFO: Sub Level
            let sub_level_children: Vec<Section> = child.children(&arena).collect();
            let mut sub_level_nodes: Vec<OrgNode> = Vec::new();

            for sub_child in sub_level_children {
                let sub_headline = sub_child.headline(&arena, None).unwrap();
                let sub_state = match sub_headline.keyword() {
                    Some(state) => state.into(),
                    None => String::from("NONE")
                };
                let sub_planning = headline.planning();

                sub_level_nodes.push(OrgNode { 
                    name: sub_headline.title().into(), 
                    state: sub_state, 
                    level: sub_headline.level().into(),
                    priority: sub_headline.priority(),
                    planning: Some(sub_headline.planning().into_owned()),
                    nodes: Some(Vec::new()) 
                });

            }

            result.push(OrgNode { 
                name: headline.title().into(), 
                state, 
                level: headline.level().into(),
                priority: headline.priority(),
                planning: Some(headline.planning().into_owned()),
                nodes: Some(sub_level_nodes) 
            });
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
        println!("result: {:?}", result);
        result
    }
}
