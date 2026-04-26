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

//! The implementation of the scope graph analyzer, which analyzes and validates the scope graph.

mod scope_traversal;

use crate::{
    ast_nodes::{ScopeID, compilation_data::ProgramContext, scope_manager::ScopeGraph},
    error::ErrorCollector,
};
use std::collections::HashMap;

/// Analyzes the scope graph and detects the recursive function calls.
pub struct ScopeGraphAnalyzer<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a ProgramContext,
    scope_graph: &'a mut ScopeGraph,
}

impl<'a> ScopeGraphAnalyzer<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a ProgramContext,
        scope_graph: &'a mut ScopeGraph,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            scope_graph,
        }
    }

    pub fn analyze_all(&mut self) {
        // Get the global scope ID
        let root_namespace_id = self.prog_ctx.namespace_registry.get_root_namespace_id();
        let global_scope_id = self
            .prog_ctx
            .scope_registry
            .get_global_scope_id(&root_namespace_id);
        // Initialize states for all scopes
        let mut states: HashMap<ScopeID, ScopeState> = self
            .prog_ctx
            .scope_registry
            .get_all_scope_ids()
            .into_iter()
            .map(|scope_id| (scope_id, ScopeState::Unvisited))
            .collect();

        // Analyze the scope graph starting from the global scope
        self.analyze_scope(&global_scope_id, &mut states);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeState {
    Unvisited,
    Visiting,
    Visited,
}
