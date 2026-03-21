use crate::{Range, VariableID, scope_manager::ScopeID};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Scope {
    pub parent: Option<ScopeID>,
    name_to_id: HashMap<String, VariableID>,
    /// Variables in this scope, in declaration order.
    pub variables: Vec<VariableID>,
    defined_names: HashSet<String>,
    pub range: Range,
}

impl Scope {
    pub fn new(parent: Option<ScopeID>, range: Range) -> Self {
        Self {
            parent,
            name_to_id: HashMap::new(),
            variables: Vec::new(),
            defined_names: HashSet::new(),
            range,
        }
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<&VariableID> {
        self.name_to_id.get(name)
    }

    pub fn has_var(&self, name: &str) -> bool {
        self.name_to_id.contains_key(name)
    }

    pub fn register_var(&mut self, name: String, id: VariableID) {
        self.name_to_id.insert(name, id);
        self.variables.push(id);
    }

    pub fn mark_name_used(&mut self, name: &str) {
        self.defined_names.insert(name.to_string());
    }

    pub fn is_name_used(&self, name: &str) -> bool {
        self.defined_names.contains(name)
    }
}
