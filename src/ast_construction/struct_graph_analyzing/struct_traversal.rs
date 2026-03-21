use crate::{
    StructID,
    error::Ph,
    struct_graph_analyzing::{StructGraphAnalyzer, StructState},
};
use std::collections::HashMap;

impl StructGraphAnalyzer<'_> {
    pub fn analyze_struct(
        &mut self,
        struct_id: &StructID,
        states: &mut HashMap<StructID, StructState>,
    ) {
        // Update the state to visiting
        states.insert(*struct_id, StructState::Visiting);

        // Analyze the scope recursively
        if let Some(struct_fields) = self.struct_graph.get_fields(struct_id) {
            for field in struct_fields {
                if states.get(field) == Some(&StructState::Visiting) {
                    if let Some(struct_decl) = self.prog_ctx.type_registry.get_struct(field) {
                        self.ec.struct_cycle(
                            struct_decl.range,
                            Ph::StructGraphAnalyzing,
                            &struct_decl.name,
                        );
                    }
                } else if states.get(field) == Some(&StructState::Unvisited) {
                    self.analyze_struct(field, states);
                }
            }
        }

        // Mark the current struct as visited
        states.insert(*struct_id, StructState::Visited);
    }
}
