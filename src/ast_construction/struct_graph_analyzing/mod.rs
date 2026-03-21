use std::collections::HashMap;

use crate::{
    StructID, compilation_data::ProgramContext, error::ErrorCollector, type_registry::StructGraph,
};
mod struct_traversal;

#[derive(Debug)]
pub struct StructGraphAnalyzer<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a ProgramContext,
    struct_graph: &'a StructGraph,
}

impl<'a> StructGraphAnalyzer<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a ProgramContext,
        struct_graph: &'a StructGraph,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            struct_graph,
        }
    }

    pub fn analyze_all(&mut self) {
        // Get the list of all StructIDs
        let all_struct_ids = self.prog_ctx.type_registry.get_all_structs();
        // Initialize states for all structs
        let mut states: HashMap<StructID, StructState> = all_struct_ids
            .iter()
            .map(|scope_id| (*scope_id, StructState::Unvisited))
            .collect();

        // Analyze the graph recursively
        for struct_id in &all_struct_ids {
            if let Some(StructState::Unvisited) = states.get(struct_id) {
                self.analyze_struct(struct_id, &mut states);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructState {
    Unvisited,
    Visiting,
    Visited,
}
