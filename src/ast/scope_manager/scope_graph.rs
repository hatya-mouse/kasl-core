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

use crate::ScopeID;

pub struct ScopeGraph {
    pub scope_sizes: HashMap<ScopeID, usize>,
    pub edges: Vec<ScopeGraphEdge>,
}

impl ScopeGraph {
    pub fn new() -> Self {
        Self {
            scope_sizes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_scope(&mut self, scope_id: ScopeID) {
        self.scope_sizes.insert(scope_id, 0);
    }

    pub fn add_edge(&mut self, caller: ScopeID, callee: ScopeID) {
        self.edges.push(ScopeGraphEdge::new(caller, callee));
    }
}

pub struct ScopeGraphEdge {
    pub caller: ScopeID,
    pub callee: ScopeID,
}

impl ScopeGraphEdge {
    pub fn new(caller: ScopeID, callee: ScopeID) -> Self {
        Self { caller, callee }
    }
}
