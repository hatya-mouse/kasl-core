//
// Copyright 2025-2026 Shuntaro Kasatani
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
    ConstructorError, ConstructorErrorType, ExprTokenKind, ParserFuncParam, SymbolPath,
    SymbolPathComponent, SymbolTable,
    resolution::{DependencyGraphNode, dependency_analysis::DependencyGraph},
};

pub fn build_func_param_graph(
    graph: &mut DependencyGraph,
    root_symbol_table: &SymbolTable,
    func_path: &SymbolPath,
    params: &[ParserFuncParam],
) -> Result<(), ConstructorError> {
    for param in params {
        // Get the default value to infer the type
        if let Some(def_value) = &param.def_val {
            for expr in def_value {
                match &expr.kind {
                    // If the default value has an identifier in it,
                    // the parameter depends on the identifier
                    ExprTokenKind::Identifier(path) => {
                        let mut from_path = func_path.clone();
                        from_path.push(SymbolPathComponent::FuncParam(param.name.clone()));

                        let to_path = match root_symbol_table.resolve_path(path) {
                            Some(path) => path,
                            None => {
                                return Err(ConstructorError {
                                    error_type: ConstructorErrorType::SymbolNotFound(None),
                                    position: expr.range,
                                });
                            }
                        };

                        graph.add_edge(&from_path, &to_path);
                        graph.add_node(DependencyGraphNode::new(from_path));
                        graph.add_node(DependencyGraphNode::new(to_path));
                    }

                    _ => (),
                }
            }
        }
    }

    Ok(())
}
