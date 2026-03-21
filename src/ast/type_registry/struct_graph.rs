use crate::StructID;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct StructGraph {
    pub parent_to_field: HashMap<StructID, HashSet<StructID>>,
}

impl StructGraph {
    pub fn add_edge(&mut self, parent: StructID, field: StructID) {
        self.parent_to_field
            .entry(parent)
            .or_default()
            .insert(field);
    }

    pub fn get_fields(&self, parent: &StructID) -> Option<&HashSet<StructID>> {
        self.parent_to_field.get(parent)
    }

    pub fn get_root_nodes(&self) -> HashSet<StructID> {
        let used_structs = self
            .parent_to_field
            .values()
            .flatten()
            .collect::<HashSet<_>>();
        self.parent_to_field
            .keys()
            .filter(|node| !used_structs.contains(node))
            .copied()
            .collect()
    }
}
