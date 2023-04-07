use orgize::{elements::Planning, Headline, Document, Org};

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
                            planning: planning.clone() 
                           }
                        );
                }
                result.push(org_section);
            }
            result
    }
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
    planning: Option<Box<Planning<'a>>>
}

// impl OrgNode {
    // pub fn new(name: String, state: OrgTodoState, level: usize, planning: Option<Box<Planning<'static>>>) -> Self {
    //     Self {
    //         name,
    //         state,
    //         level,
    //         planning
    //     }
    // }
// }
