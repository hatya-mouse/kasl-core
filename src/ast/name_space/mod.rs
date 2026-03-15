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
mod reserved_type_names;
mod symbol_id;
mod symbol_path;

pub use import_path::ImportPath;
pub use reserved_type_names::is_reserved_type_name;
pub use symbol_id::{FunctionID, NameSpaceID, OperatorID, ParserStmtID, StructID, VariableID};
pub use symbol_path::{SymbolPath, SymbolPathComponent};

use crate::{
    OperatorContext, ScopeRegistry, symbol_table::FunctionContext, type_registry::TypeRegistry,
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct NameSpaceRegistry {
    pub namespaces: HashMap<NameSpaceID, NameSpace>,
    pub path_to_id: HashMap<Vec<String>, NameSpaceID>,
    root_namespace_id: NameSpaceID,
    next_namespace_id: usize,
}

impl NameSpaceRegistry {
    pub fn new() -> Self {
        let mut registry = Self::default();
        // Register the root namespace
        registry.root_namespace_id = registry.generate_namespace_id();
        registry
            .namespaces
            .insert(registry.root_namespace_id, NameSpace::default());
        registry
    }

    pub fn get_root_namespace_id(&self) -> NameSpaceID {
        self.root_namespace_id
    }

    pub fn get_namespace_by_id(&self, id: NameSpaceID) -> Option<&NameSpace> {
        self.namespaces.get(&id)
    }

    pub fn register_namespace(&mut self, import_path: &ImportPath, namespace: NameSpace) {
        let namespace_id = self.generate_namespace_id();
        self.namespaces.insert(namespace_id, namespace);
        self.path_to_id
            .insert(import_path.path.clone(), namespace_id);
    }

    pub fn generate_namespace_id(&mut self) -> NameSpaceID {
        let id = NameSpaceID::new(self.next_namespace_id);
        self.next_namespace_id += 1;
        id
    }

    pub fn get_id_by_path(&self, path: &[String]) -> Option<NameSpaceID> {
        self.path_to_id.get(path).copied()
    }
}

#[derive(Debug, Default)]
pub struct NameSpace {
    pub func_ctx: FunctionContext,
    pub op_ctx: OperatorContext,
    pub scope_registry: ScopeRegistry,
    pub type_registry: TypeRegistry,
    pub namespace_registry: NameSpaceRegistry,
}
