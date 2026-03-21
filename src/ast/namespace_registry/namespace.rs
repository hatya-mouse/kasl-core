use crate::NameSpaceID;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct NameSpace {
    pub id: NameSpaceID,
    child_namespaces: HashMap<String, NameSpaceID>,
}

impl NameSpace {
    pub fn new(id: NameSpaceID) -> Self {
        Self {
            id,
            child_namespaces: HashMap::new(),
        }
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<NameSpaceID> {
        self.child_namespaces.get(name).copied()
    }

    pub fn add_child(&mut self, name: String, id: NameSpaceID) {
        self.child_namespaces.insert(name, id);
    }
}
