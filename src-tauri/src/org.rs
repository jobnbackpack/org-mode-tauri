use orgize::{elements::Planning, Document, Org};

#[derive(Debug, serde::Serialize)]
pub struct OrgSection<'a> {
    pub title: String,
    pub nodes: Vec<OrgNode<'a>>
}

impl OrgSection<'_> {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            nodes: Vec::new()
        }
    }

    pub fn new_section_vec_from_document(document: Document, org: Org) -> Vec<OrgSection> {
            let top_level_headings = document.children(&org);

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
                    let node_title = &node.title(&org).clone();
                    let title = node_title.raw.to_string();
                    let planning = &node_title.planning;

                    org_section.nodes.push(
                        OrgNode {
                            name: title,
                            state,
                            level: node.level(),
                            planning: planning.clone(),
                            priority: *&node_title.priority,
                            nodes: None
                           }
                        );
                }
                result.push(org_section);
            }
            result
    }

}

#[derive(Debug, serde::Serialize)]
pub struct OrgFile {
    pub name: String,
    pub path: String,
    pub nodes: Vec<OrgNode<'static>>
}

#[derive(Debug, serde::Serialize)]
pub enum OrgTodoState {
    TODO,
    DONE,
    NONE
}

#[derive(Debug, serde::Serialize)]
pub struct OrgNode<'a> {
    name: String,
    state: OrgTodoState,
    level: usize,
    priority: Option<char>,
    planning: Option<Box<Planning<'a>>>,
    nodes: Option<Vec<OrgNode<'a>>>
}

impl OrgNode<'_> {
    pub fn get_all_nodes_from_doc(document: Document, org: Org) -> Vec<OrgNode> {
        let top_level_nodes = document.children(&org);

        let mut result: Vec<OrgNode> = Vec::new();

        for node in top_level_nodes {
            let state = match node.title(&org).keyword.as_deref() {
                Some("TODO") => OrgTodoState::TODO,
                Some("DONE") => OrgTodoState::DONE,
                None => OrgTodoState::NONE,
                _ => OrgTodoState::NONE
            };
            let node_title = &node.title(&org).clone();
            let title = node_title.raw.to_string();
            let planning = &node_title.planning;
            let priority = &node_title.priority;
            let mut sub_nodes: Vec<OrgNode> = Vec::new();

            for sub_node in node.children(&org) {
                let state = match sub_node.title(&org).keyword.as_deref() {
                    Some("TODO") => OrgTodoState::TODO,
                    Some("DONE") => OrgTodoState::DONE,
                    None => OrgTodoState::NONE,
                    _ => OrgTodoState::NONE
                };
                let node_title = &sub_node.title(&org).clone();
                let title = node_title.raw.to_string();
                let planning = &node_title.planning;
                let priority = &node_title.priority;
                let mut sub_sub_nodes: Vec<OrgNode> = Vec::new();

                for sub_sub_node in sub_node.children(&org) {
                    let state = match sub_sub_node.title(&org).keyword.as_deref() {
                        Some("TODO") => OrgTodoState::TODO,
                        Some("DONE") => OrgTodoState::DONE,
                        None => OrgTodoState::NONE,
                        _ => OrgTodoState::NONE
                    };
                    let node_title = &sub_sub_node.title(&org).clone();
                    let title = node_title.raw.to_string();
                    let planning = &node_title.planning;
                    let priority = &node_title.priority;

                    sub_sub_nodes.push(
                        OrgNode {
                            name: title,
                            state,
                            level: sub_sub_node.level(),
                            planning: planning.clone(),
                            priority: *priority,
                            nodes: None
                        }
                        );
                }
                sub_nodes.push(
                    OrgNode {
                        name: title,
                        state,
                        level: sub_node.level(),
                        planning: planning.clone(),
                        priority: *priority,
                        nodes: Some(sub_sub_nodes)
                    }
                    );
            }

            result.push(
                OrgNode {
                    name: title,
                    state,
                    level: node.level(),
                    planning: planning.clone(),
                    priority: *priority,
                    nodes: Some(sub_nodes)
                }
                );
        }
        result
    }
}
