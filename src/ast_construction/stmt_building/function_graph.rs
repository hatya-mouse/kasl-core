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

use crate::data::SymbolID;

/// A function dependency graph to detect recursive function call.
pub struct FunctionGraph {
    pub nodes: Vec<SymbolID>,
    /// A map from a function ID to its outgoing edges.
    /// The key is an ID of the function that calls the function with an ID in the value.
    pub edges: HashMap<SymbolID, Vec<SymbolID>>,
}

impl FunctionGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: SymbolID, to: SymbolID) {
        self.edges.entry(from).or_default().push(to);
    }

    pub fn add_node(&mut self, id: SymbolID) {
        self.nodes.push(id);
    }
}
