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
    ExprToken, ExprTokenKind, SymbolPath, SymbolPathComponent, SymbolTable,
    error::{ErrorCollector, Phase},
    resolution::{DependencyGraphNode, dependency_analysis::DependencyGraph},
};

pub fn build_var_graph(
    ec: &mut ErrorCollector,
    graph: &mut DependencyGraph,
    root_symbol_table: &SymbolTable,
    var_path: &SymbolPath,
    def_val: &Vec<ExprToken>,
) {
    // If the default value has any identifiers, thus the variable depends on them
    for expr in def_val {
        match &expr.kind {
            ExprTokenKind::Identifier(path) => {
                let resolved_path = match root_symbol_table.resolve_path(path) {
                    Some(path) => path,
                    None => {
                        ec.var_not_found(expr.range, Phase::GraphConstruction, &path.to_string());
                        return;
                    }
                };

                // Normalize the path to remove unnecessary components
                // Because we just need to infer the type of the top variable or function
                let mut to_path = SymbolPath::new();
                for component in resolved_path.components {
                    match component {
                        SymbolPathComponent::Var(_) | SymbolPathComponent::Func(_) => {
                            // If the path reached a variable or a function, stop here
                            to_path.push(component);
                            break;
                        }
                        _ => to_path.push(component),
                    }
                }

                if !to_path.components.is_empty() {
                    graph.add_edge(var_path, &to_path);
                    graph.add_node(DependencyGraphNode::new(to_path));
                }
            }

            ExprTokenKind::FuncCall { path, .. } => {
                let to_path = match root_symbol_table.resolve_path(path) {
                    Some(path) => path,
                    None => {
                        ec.func_not_found(expr.range, Phase::GraphConstruction, &path.to_string());
                        return;
                    }
                };

                graph.add_edge(var_path, &to_path);
                graph.add_node(DependencyGraphNode::new(var_path.clone()));
                graph.add_node(DependencyGraphNode::new(to_path));
            }

            _ => (),
        }
    }
}
