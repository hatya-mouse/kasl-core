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
    ParserTopLevelStmtKind, SymbolTable,
    error::ErrorCollector,
    resolution::{
        DependencyGraphNode,
        dependency_analysis::{DependencyGraph, build_func_param_graph, build_var_graph},
    },
};

pub fn build_graph(ec: &mut ErrorCollector, symbol_table: &SymbolTable) -> Option<DependencyGraph> {
    let mut graph = DependencyGraph::new();

    // Output variables MUST have type annotations therefore we don't need to resolve their types.
    for (symbol_id, stmt) in &symbol_table.get_tuples() {
        match &stmt.kind {
            ParserTopLevelStmtKind::ScopeVar { def_val, .. }
            | ParserTopLevelStmtKind::Input { def_val, .. }
            | ParserTopLevelStmtKind::Output { def_val, .. }
            | ParserTopLevelStmtKind::StateVar { def_val, .. } => {
                build_var_graph(ec, &mut graph, symbol_table, *symbol_id, def_val);
                graph.add_node(DependencyGraphNode::new(*symbol_id));
            }

            ParserTopLevelStmtKind::StructDecl { .. } => {
                graph.add_node(DependencyGraphNode::new(*symbol_id));
            }

            ParserTopLevelStmtKind::OperatorFunc { params, .. }
            | ParserTopLevelStmtKind::FuncDecl { params, .. } => {
                build_func_param_graph(ec, &mut graph, symbol_table, *symbol_id, params);
                graph.add_node(DependencyGraphNode::new(*symbol_id));
            }

            ParserTopLevelStmtKind::InfixDefine { .. } => {
                graph.add_node(DependencyGraphNode::new(*symbol_id));
            }
        }
    }

    Some(graph)
}
