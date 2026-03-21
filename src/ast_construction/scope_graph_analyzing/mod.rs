mod scope_traversal;

use crate::{
    ScopeID, compilation_data::ProgramContext, error::ErrorCollector, scope_manager::ScopeGraph,
};
use std::collections::HashMap;

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
