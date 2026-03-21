mod io_blueprint;
mod reserved_type_names;
mod scope;
mod scope_graph;
mod scope_var;

pub use io_blueprint::{BlueprintItem, IOBlueprint};
pub use scope::Scope;
pub use scope_graph::ScopeGraph;
pub use scope_var::{InputAttribute, ScopeVar, VariableKind};

pub(crate) use io_blueprint::BlueprintItemKind;

use crate::{NameSpaceID, Range, VariableID, scope_manager::reserved_type_names::is_reserved_name};
use std::{collections::HashMap, fmt::Display};

/// ScopeRegistry manages scopes and variables belonging to them.
/// It only manages the top-level variables and local variables,
/// and doesn't manage the struct fields.
#[derive(Default, Debug, serde::Serialize)]
pub struct ScopeRegistry {
    scopes: HashMap<ScopeID, Scope>,
    variables: HashMap<VariableID, ScopeVar>,
    global_scope_ids: HashMap<NameSpaceID, ScopeID>,
    next_scope_id: usize,
    next_variable_id: usize,
}

impl ScopeRegistry {
    // --- SCOPE CREATION ---

    /// Creates a new global scope for the given namespace ID.
    pub fn create_global_scope(&mut self, namespace_id: NameSpaceID) {
        let global_scope_id = self.create_scope(None, Range::zero());
        self.global_scope_ids.insert(namespace_id, global_scope_id);
    }

    /// Creates a new scope with the given parent scope ID and the range.
    pub fn create_scope(&mut self, parent_scope_id: Option<ScopeID>, range: Range) -> ScopeID {
        let scope_id = self.generate_scope_id();
        let scope = Scope::new(parent_scope_id, range);
        self.scopes.insert(scope_id, scope);
        scope_id
    }

    /// Generates a new `ScopeID` for a new scope.
    pub fn generate_scope_id(&mut self) -> ScopeID {
        let id = ScopeID(self.next_scope_id);
        self.next_scope_id += 1;
        id
    }

    /// Generates a new `VariableID` for a new variable.
    pub fn generate_var_id(&mut self) -> VariableID {
        let id = VariableID(self.next_variable_id);
        self.next_variable_id += 1;
        id
    }

    // --- GETTER FUNCTIONS ---

    /// Gets the global scope ID for the given namespace ID.
    pub fn get_global_scope_id(&self, namespace_id: &NameSpaceID) -> ScopeID {
        self.global_scope_ids[namespace_id]
    }

    /// Gets a reference to the global scope for the given namespace ID.
    pub fn get_global_scope(&self, namespace_id: &NameSpaceID) -> &Scope {
        let scope_id = self.get_global_scope_id(namespace_id);
        &self.scopes[&scope_id]
    }

    /// Gets the `VariableID` of the variable in the given scope or its parent scopes with the given name.
    pub fn get_var_id(&self, scope_id: ScopeID, name: &str) -> Option<VariableID> {
        let mut target = Some(scope_id);
        while let Some(scope_id) = target {
            let scope = &self.scopes[&scope_id];
            if let Some(var_id) = scope.get_id_by_name(name) {
                return Some(*var_id);
            }
            target = scope.parent;
        }
        None
    }

    /// Gets a reference to the variable with the given `VariableID`.
    pub fn get_var(&self, var_id: &VariableID) -> Option<&ScopeVar> {
        self.variables.get(var_id)
    }

    /// Returns a reference to the scope with the given `ScopeID`.
    pub fn get_scope(&self, scope_id: &ScopeID) -> Option<&Scope> {
        self.scopes.get(scope_id)
    }

    /// Gets IDs of all scopes.
    pub fn get_all_scope_ids(&self) -> Vec<ScopeID> {
        self.scopes.keys().copied().collect()
    }

    // --- REGISTRATION ---

    /// Registers a new variable in the given scope with the given name and returns its `VariableID`.
    pub fn register_var(&mut self, var: ScopeVar, name: String, scope_id: &ScopeID) -> VariableID {
        let var_id = self.generate_var_id();
        self.variables.insert(var_id, var);
        self.scopes
            .get_mut(scope_id)
            .unwrap()
            .register_var(name, var_id);
        var_id
    }

    // --- NAME DUPLICATION DETECTION ---

    /// Add the name to the set of defined names for the given namespace.
    pub fn mark_name_used(&mut self, scope_id: &ScopeID, name: &str) {
        if let Some(scope) = self.scopes.get_mut(scope_id) {
            scope.mark_name_used(name)
        }
    }

    /// Returns if the name is already used.
    /// Returns `true` if the namespace is not found.
    pub fn is_name_used(&self, scope_id: &ScopeID, name: &str) -> bool {
        self.get_scope(scope_id)
            .is_none_or(|scope| scope.is_name_used(name))
            || is_reserved_name(name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct ScopeID(usize);

impl Display for ScopeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
