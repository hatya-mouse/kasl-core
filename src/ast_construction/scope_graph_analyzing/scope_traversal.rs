//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{
    ast::ScopeID,
    ast_construction::scope_graph_analyzing::{ScopeGraphAnalyzer, ScopeState},
    error::Ph,
};
use std::collections::HashMap;

impl ScopeGraphAnalyzer<'_> {
    pub fn analyze_scope(
        &mut self,
        current_scope: &ScopeID,
        states: &mut HashMap<ScopeID, ScopeState>,
    ) {
        // Update the state to Visiting
        states.insert(*current_scope, ScopeState::Visiting);

        // Analyze the scope recursively
        let child_scopes = self.scope_graph.get_callees(current_scope).cloned();

        if let Some(child_scopes) = &child_scopes {
            for child_scope_id in child_scopes {
                if states.get(child_scope_id) == Some(&ScopeState::Visiting) {
                    let child_scope_range = self
                        .prog_ctx
                        .scope_registry
                        .get_scope(child_scope_id)
                        .map(|scope| scope.range)
                        .unwrap_or_default();
                    self.ec
                        .recursive_call(child_scope_range, Ph::ScopeGraphAnalyzing);
                } else {
                    self.analyze_scope(child_scope_id, states);
                }
            }
        }

        // Mark the current scope as Visited
        states.insert(*current_scope, ScopeState::Visited);
    }
}
