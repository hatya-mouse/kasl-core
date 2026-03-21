use crate::ScopeID;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct ScopeGraph {
    pub caller_to_callee: HashMap<ScopeID, HashSet<ScopeID>>,
    /// Whether the scope guarantees return statement.
    scope_has_return: HashMap<ScopeID, bool>,
    scope_requires_return: HashMap<ScopeID, bool>,
}

impl ScopeGraph {
    pub fn add_edge(&mut self, caller: ScopeID, callee: ScopeID) {
        self.caller_to_callee
            .entry(caller)
            .or_default()
            .insert(callee);
    }

    pub fn get_callees(&self, caller: &ScopeID) -> Option<&HashSet<ScopeID>> {
        self.caller_to_callee.get(caller)
    }

    pub fn guarantees_return(&self, id: &ScopeID) -> bool {
        *self.scope_has_return.get(id).unwrap_or(&false)
    }

    pub fn requires_return(&self, id: &ScopeID) -> bool {
        *self.scope_requires_return.get(id).unwrap_or(&false)
    }

    pub fn set_has_return(&mut self, id: ScopeID, has_return: bool) {
        self.scope_has_return.insert(id, has_return);
    }

    pub fn set_requires_return(&mut self, id: ScopeID, requires_return: bool) {
        self.scope_requires_return.insert(id, requires_return);
    }
}
