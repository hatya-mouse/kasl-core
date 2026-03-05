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

use std::collections::HashMap;

use crate::{
    Function, InfixOperator, InfixOperatorProperties, InputVar, OutputVar, PrefixOperator,
    PrimitiveType, ScopeVar, StateVar, SymbolPath,
    data::{SymbolID, SymbolMetadata},
    tree_items::StructDecl,
};

#[derive(Debug)]
pub struct Program {
    pub funcs: HashMap<SymbolID, Function>,
    pub structs: HashMap<SymbolID, StructDecl>,
    pub inputs: HashMap<SymbolID, InputVar>,
    pub outputs: HashMap<SymbolID, OutputVar>,
    pub states: HashMap<SymbolID, StateVar>,
    pub vars: HashMap<SymbolID, ScopeVar>,
    pub infix_operators: HashMap<SymbolID, InfixOperator>,
    pub prefix_operators: HashMap<SymbolID, PrefixOperator>,
    pub primitive_types: HashMap<SymbolID, PrimitiveType>,

    pub infix_operator_properties: HashMap<String, InfixOperatorProperties>,

    id_to_path: HashMap<SymbolID, SymbolPath>,
    path_to_id: HashMap<SymbolPath, Vec<SymbolID>>,
    metadata: HashMap<SymbolID, SymbolMetadata>,
    next_id: usize,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            structs: HashMap::new(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            states: HashMap::new(),
            vars: HashMap::new(),
            infix_operators: HashMap::new(),
            prefix_operators: HashMap::new(),
            primitive_types: HashMap::new(),

            infix_operator_properties: HashMap::new(),

            id_to_path: HashMap::new(),
            path_to_id: HashMap::new(),
            metadata: HashMap::new(),
            next_id: 0,
        }
    }

    /// Returns a SymbolID for the given SymbolPath.
    pub fn get_id_by_path(&self, path: &SymbolPath) -> Option<&Vec<SymbolID>> {
        self.path_to_id.get(path)
    }

    /// Returns a SymbolPath for the given SymbolID.
    pub fn get_path_by_id(&self, id: &SymbolID) -> Option<&SymbolPath> {
        self.id_to_path.get(id)
    }

    /// Registers a new SymbolID for the given SymbolPath and category.
    pub fn register_id(&mut self, path: SymbolPath, id: SymbolID) {
        self.path_to_id.entry(path.clone()).or_default().push(id);
        self.id_to_path.insert(id, path);
    }

    /// Returns a next available SymbolID for the given SymbolPath and category.
    pub fn get_next_id(&mut self) -> SymbolID {
        let id = SymbolID::new(self.next_id);
        self.next_id += 1;
        id
    }

    /// Returns the type of the given SymbolID.
    pub fn get_symbol_type(&self, id: &SymbolID) -> Option<SymbolID> {
        self.metadata
            .get(id)
            .and_then(|metadata| metadata.symbol_type)
    }

    /// Adds metadata for the given SymbolID.
    pub fn add_metadata(&mut self, id: SymbolID, metadata: SymbolMetadata) {
        self.metadata.insert(id, metadata);
    }
}
