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

mod import_path;
mod namespace;
mod namespace_pair;
mod reserved_type_names;
mod symbol_id;
mod symbol_path;

pub use import_path::ImportPath;
pub use namespace::NameSpace;
pub use namespace_pair::NameSpacePair;
pub use symbol_id::{FunctionID, NameSpaceID, OperatorID, ParserStmtID, StructID, VariableID};
pub use symbol_path::{SymbolPath, SymbolPathComponent};

use crate::namespace_registry::reserved_type_names::is_reserved_name;
use std::collections::HashMap;

/// Stores a id-namespace pair of all namespaces in compilation.
/// Should only exist one instance per compilation.
#[derive(Debug, Default)]
pub struct NameSpaceRegistry {
    pub namespaces: HashMap<NameSpaceID, NameSpace>,
    root_namespace_id: NameSpaceID,
    next_namespace_id: usize,
}

impl NameSpaceRegistry {
    /// Creates a new `NameSpaceRegistry` with the root namespace registered.
    pub fn new() -> Self {
        let mut registry = Self::default();
        // Register the root namespace
        registry.root_namespace_id = registry.generate_namespace_id();
        registry
            .namespaces
            .insert(registry.root_namespace_id, NameSpace::default());
        registry
    }

    pub fn generate_namespace_id(&mut self) -> NameSpaceID {
        let id = NameSpaceID::new(self.next_namespace_id);
        self.next_namespace_id += 1;
        id
    }

    // --- GETTER FUNCTIONS ---

    pub fn get_root_namespace_id(&self) -> NameSpaceID {
        self.root_namespace_id
    }

    pub fn get_namespace_by_id(&self, id: &NameSpaceID) -> Option<&NameSpace> {
        self.namespaces.get(id)
    }

    // --- REGISTRATION ---

    pub fn register_namespace(
        &mut self,
        name: String,
        namespace: NameSpace,
        parent: Option<NameSpaceID>,
    ) {
        let namespace_id = self.generate_namespace_id();
        self.namespaces.insert(namespace_id, namespace);
        if let Some(parent) = parent {
            self.namespaces
                .get_mut(&parent)
                .unwrap()
                .add_child(name, namespace_id);
        }
    }

    // --- NAME DUPLICATION DETECTION ---

    /// Add the name to the set of defined names for the given namespace.
    pub fn mark_as_defined(&mut self, namespace_id: &NameSpaceID, name: String) {
        self.namespaces
            .get_mut(namespace_id)
            .map(|namespace| namespace.mark_as_defined(name));
    }

    /// Returns if the name is already used.
    /// Returns `true` if the namespace is not found.
    pub fn is_name_used(&mut self, namespace_id: &NameSpaceID, name: &str) -> bool {
        self.get_namespace_by_id(namespace_id)
            .is_none_or(|namespace| namespace.is_name_used(name))
            || is_reserved_name(name)
    }

    // --- NAMESPACE RESOLUTION ---

    /// Resolves a namespace from a first few path components and returns the corresponding namespace and the remaining path components.
    pub fn resolve_namespace_from_path(&self, path: SymbolPath) -> (NameSpaceID, SymbolPath) {
        let mut path_iter = path.into_iter().peekable();
        let mut current_namespace_id = self.root_namespace_id;
        while let Some(name) = path_iter.peek() {
            if let Some(namespace) = self.get_namespace_by_id(&current_namespace_id) {
                // Get the child namespace ID by name
                if let Some(child_id) = namespace.get_id_by_name(&name.symbol) {
                    path_iter.next();
                    current_namespace_id = child_id;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        (current_namespace_id, path_iter.collect())
    }
}
