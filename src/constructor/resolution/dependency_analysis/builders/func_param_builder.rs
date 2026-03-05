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
    ExprTokenKind, ParserFuncParam, SymbolTable,
    data::ParserStmtID,
    error::{ErrorCollector, Phase},
    resolution::{DependencyGraphNode, dependency_analysis::DependencyGraph},
};

pub fn build_func_param_graph(
    ec: &mut ErrorCollector,
    graph: &mut DependencyGraph,
    symbol_table: &SymbolTable,
    func_id: ParserStmtID,
    params: &[ParserFuncParam],
) {
    for param in params {
        // Get the default value to infer the type
        if let Some(def_value) = &param.def_val {
            for expr in def_value {
                match &expr.kind {
                    // If the default value has an identifier in it,
                    // the parameter depends on the identifier
                    ExprTokenKind::Identifier(path) => {
                        let to_ids = match symbol_table.get_id_by_path(path) {
                            Some(ids) => ids,
                            None => {
                                ec.var_not_found(
                                    expr.range,
                                    Phase::GraphConstruction,
                                    &path.to_string(),
                                );
                                return;
                            }
                        };

                        graph.add_node(DependencyGraphNode::new(func_id));
                        for to_id in to_ids {
                            graph.add_edge(func_id, *to_id);
                            graph.add_node(DependencyGraphNode::new(*to_id));
                        }
                    }

                    ExprTokenKind::FuncCall { path, .. } => {
                        let to_ids = match symbol_table.get_id_by_path(path) {
                            Some(ids) => ids,
                            None => {
                                ec.func_not_found(
                                    expr.range,
                                    Phase::GraphConstruction,
                                    &path.to_string(),
                                );
                                return;
                            }
                        };

                        graph.add_node(DependencyGraphNode::new(func_id));
                        for to_id in to_ids {
                            graph.add_edge(func_id, *to_id);
                            graph.add_node(DependencyGraphNode::new(*to_id));
                        }
                    }

                    _ => (),
                }
            }
        }
    }
}
