//
// © 2025-2026 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

mod scope;
mod scope_var;

pub use scope::Scope;
pub use scope_var::{ScopeVar, VariableKind};

use crate::VariableID;
use std::collections::HashMap;

pub struct ScopeRegistry {
    pub scopes: HashMap<ScopeID, Scope>,
    variables: HashMap<VariableID, ScopeVar>,
    global_scope_id: ScopeID,
    next_id: usize,
}

impl ScopeRegistry {
    /// Creates a new `ScopeRegistry` with a new global scope.
    pub fn new() -> Self {
        let mut manager = Self {
            scopes: HashMap::new(),
            variables: HashMap::new(),
            global_scope_id: ScopeID(0),
            next_id: 0,
        };
        // Create the global scope
        manager.global_scope_id = manager.create_scope(None);
        manager
    }

    /// Returns a mutable reference to the global scope.
    pub fn get_global_scope_mut(&mut self) -> &mut Scope {
        self.scopes.get_mut(&self.global_scope_id).unwrap()
    }

    /// Returns a reference to the scope with the given `ScopeID`.
    pub fn get_scope(&self, scope_id: ScopeID) -> Option<&Scope> {
        self.scopes.get(&scope_id)
    }

    /// Generates a new `ScopeID` for a new scope.
    pub fn generate_id(&mut self) -> ScopeID {
        let id = self.next_id;
        self.next_id += 1;
        ScopeID(id)
    }

    /// Creates a new scope with the given parent scope.
    pub fn create_scope(&mut self, parent: Option<ScopeID>) -> ScopeID {
        let id = self.generate_id();
        let scope = Scope::new(parent);
        self.scopes.insert(id, scope);
        id
    }

    /// Looks up a variable by name in the current scope and its parents.
    pub fn lookup_var(&self, current_scope: ScopeID, name: &str) -> Option<&VariableID> {
        let mut target = Some(current_scope);
        while let Some(scope_id) = target {
            let scope = &self.scopes[&scope_id];
            if let Some(symbol_id) = scope.get_id_by_name(name) {
                return Some(symbol_id);
            }
            target = scope.parent;
        }
        None
    }

    /// Returns a reference to the variable by ID.
    pub fn get_var_by_id(&self, id: &VariableID) -> Option<&ScopeVar> {
        self.variables.get(id)
    }

    // Registers a variable in the scope registry.
    pub fn register_var(&mut self, var: ScopeVar, name: String, id: VariableID, scope: ScopeID) {
        let target_scope = self.scopes.get_mut(&scope).unwrap();
        target_scope.register_var(name, id);
        self.variables.insert(id, var);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ScopeID(usize);
