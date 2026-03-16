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
