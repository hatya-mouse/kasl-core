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

use crate::{
    Range, ScopeID,
    error::Ph,
    scope_graph_analyzing::{ScopeGraphAnalyzer, ScopeState},
};
use std::{cmp::max, collections::HashMap};

impl ScopeGraphAnalyzer<'_> {
    pub fn analyze_scope(
        &mut self,
        current_scope: &ScopeID,
        states: &mut HashMap<ScopeID, ScopeState>,
        total_sizes: &mut HashMap<ScopeID, i32>,
    ) {
        // Update the state to Visiting
        states.insert(*current_scope, ScopeState::Visiting);

        // Calculate the memory layout of the current scope
        let current_scope_size = self.calculate_scope_layout(current_scope);

        // Analyze the scope recursively
        let mut max_child_size: i32 = 0;
        if let Some(child_scopes) = self.scope_graph.get_callees(current_scope) {
            for child_scope in child_scopes {
                if states.get(child_scope) == Some(&ScopeState::Visiting) {
                    self.ec
                        .recursive_call(Range::zero(), Ph::ScopeGraphAnalyzing);
                } else {
                    self.analyze_scope(child_scope, states, total_sizes);
                    // Take the maximum of the current total child size and the size of the child scope
                    // Scopes with the same level should not exist at the same type
                    max_child_size = max(max_child_size, total_sizes[child_scope]);
                }
            }
        }

        // Add the current scope size and the maximum child size to get the maximum scope size in bytes
        let total_size = max_child_size + current_scope_size;

        // Mark the current scope as Visited
        states.insert(*current_scope, ScopeState::Visited);
        // Update the total size of the current scope
        total_sizes.insert(*current_scope, total_size);
    }

    pub fn calculate_scope_layout(&mut self, scope_id: &ScopeID) -> i32 {
        let mut size = 0;
        if let Some(scope) = self.scope_registry.get_scope(scope_id) {
            for var_id in &scope.variables {
                let Some(var) = self.scope_registry.get_var_by_id(var_id) else {
                    continue;
                };

                // Get the size of the variable type
                let var_size = self.type_registry.get_type_size(&var.def_val.value_type);
                // Update the total size of the scope
                size += var_size;
            }
        }
        size
    }
}
