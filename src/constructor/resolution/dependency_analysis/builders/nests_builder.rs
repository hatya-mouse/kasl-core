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
    ParserStatementKind, SymbolPath, SymbolPathComponent, SymbolTable,
    error::ErrorCollector,
    resolution::dependency_analysis::{DependencyGraph, build_func_param_graph, build_var_graph},
};

pub fn build_struct_and_protocol_graph(
    ec: &mut ErrorCollector,
    graph: &mut DependencyGraph,
    type_path: &SymbolPath,
    root_symbol_table: &SymbolTable,
    child_symbol_table: &SymbolTable,
) {
    for stmt in &child_symbol_table.vars {
        if let ParserStatementKind::Var {
            required_by: _,
            name,
            value_type: _,
            def_val,
        } = &stmt.1.kind
        {
            // Combine variable name to create a new path for the child type
            let mut var_path = type_path.clone();
            var_path.push(SymbolPathComponent::Var(name.to_string()));
            build_var_graph(ec, graph, root_symbol_table, &var_path, def_val);
        }
    }

    for stmt in &child_symbol_table.funcs {
        if let ParserStatementKind::FuncDecl {
            required_by: _,
            name,
            params,
            return_type: _,
            body: _,
        } = &stmt.1.kind
        {
            // Combine function name to create a new path for the function
            let mut func_path = type_path.clone();
            func_path.push(SymbolPathComponent::Func(name.to_string()));
            build_func_param_graph(ec, graph, root_symbol_table, &func_path, params);
        }
    }

    for stmt in &child_symbol_table.type_defs {
        match &stmt.1.0.kind {
            ParserStatementKind::StructDecl {
                name,
                inherits: _,
                body: _,
            }
            | ParserStatementKind::ProtocolDecl {
                name,
                inherits: _,
                body: _,
            } => {
                if let Some(decl_expr) = child_symbol_table.get_type_def(name) {
                    let child_symbol_table = &decl_expr.1;

                    // Combine the child type name to create a new type path for the child type
                    let mut child_type_path = type_path.clone();
                    child_type_path.push(SymbolPathComponent::TypeDef(name.to_string()));

                    build_struct_and_protocol_graph(
                        ec,
                        graph,
                        &child_type_path,
                        root_symbol_table,
                        child_symbol_table,
                    );
                }
            }

            _ => (),
        }
    }
}
