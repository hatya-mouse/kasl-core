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

mod scope_traversal;

use std::collections::HashMap;

use crate::{NameSpace, ScopeID, error::ErrorCollector, scope_manager::ScopeGraph};

pub struct ScopeGraphAnalyzer<'a> {
    ec: &'a mut ErrorCollector,
    namespace: &'a NameSpace,
    scope_graph: &'a mut ScopeGraph,
}

impl<'a> ScopeGraphAnalyzer<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        namespace: &'a NameSpace,
        scope_graph: &'a mut ScopeGraph,
    ) -> Self {
        Self {
            ec,
            namespace,
            scope_graph,
        }
    }

    pub fn analyze_all(&mut self) {
        // Get the global scope ID
        let global_scope_id = self.namespace.scope_registry.get_global_scope_id();
        // Initialize states for all scopes
        let mut states: HashMap<ScopeID, ScopeState> = self
            .namespace
            .scope_registry
            .all_scope_ids()
            .into_iter()
            .map(|scope_id| (scope_id, ScopeState::Unvisited))
            .collect();
        // Create a total sizes map for all scopes
        let mut total_sizes: HashMap<ScopeID, usize> = HashMap::new();

        // Analyze the scope graph starting from the global scope
        self.analyze_scope(&global_scope_id, &mut states, &mut total_sizes);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeState {
    Unvisited,
    Visiting,
    Visited,
}
