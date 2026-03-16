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
