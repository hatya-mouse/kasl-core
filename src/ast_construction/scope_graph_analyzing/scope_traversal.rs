use crate::{
    ScopeID,
    error::Ph,
    scope_graph_analyzing::{ScopeGraphAnalyzer, ScopeState},
};
use std::{cmp::max, collections::HashMap};

impl ScopeGraphAnalyzer<'_> {
    pub fn analyze_scope(
        &mut self,
        current_scope: &ScopeID,
        states: &mut HashMap<ScopeID, ScopeState>,
        total_sizes: &mut HashMap<ScopeID, usize>,
    ) {
        // Update the state to Visiting
        states.insert(*current_scope, ScopeState::Visiting);

        // Calculate the size of the current scope
        let current_scope_size = self.calculate_scope_size(current_scope);

        // Analyze the scope recursively
        let mut max_child_size: usize = 0;
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
                    self.analyze_scope(child_scope_id, states, total_sizes);

                    // Take the maximum of the current total child size and the size of the child scope
                    // Scopes with the same level should not exist at the same type
                    max_child_size = max(max_child_size, total_sizes[child_scope_id]);
                }
            }
        }

        // Check if this scope guarantees return
        let current_scope_has_return = self.scope_graph.guarantees_return(current_scope);
        let all_children_have_return = child_scopes
            .map(|children| {
                children
                    .iter()
                    .all(|child| self.scope_graph.guarantees_return(child))
            })
            .unwrap_or(false);
        let guarantees_return = current_scope_has_return || all_children_have_return;
        // If the current scope requires return but doesn't guarantee return, throw an error
        if self.scope_graph.requires_return(current_scope) && !guarantees_return {
            let scope_range = self
                .prog_ctx
                .scope_registry
                .get_scope(current_scope)
                .map(|scope| scope.range)
                .unwrap_or_default();
            self.ec.missing_return(scope_range, Ph::ScopeGraphAnalyzing);
        }
        // Set if the current scope guarantees return
        self.scope_graph
            .set_has_return(*current_scope, guarantees_return);

        // Add the current scope size and the maximum child size to get the maximum scope size in bytes
        let total_size = max_child_size + current_scope_size;

        // Mark the current scope as Visited
        states.insert(*current_scope, ScopeState::Visited);
        // Update the total size of the current scope
        total_sizes.insert(*current_scope, total_size);
    }

    fn calculate_scope_size(&mut self, scope_id: &ScopeID) -> usize {
        let mut size = 0;
        if let Some(scope) = self.prog_ctx.scope_registry.get_scope(scope_id) {
            for var_id in &scope.variables {
                let Some(var) = self.prog_ctx.scope_registry.get_var(var_id) else {
                    continue;
                };

                // Get the size of the variable type
                let var_size = self
                    .prog_ctx
                    .type_registry
                    .get_type_actual_size(&var.value_type)
                    .unwrap();
                // Update the total size of the scope
                size += var_size;
            }
        }
        size
    }
}
