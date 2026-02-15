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
    ParserStatement, ParserStatementKind, SymbolPathComponent, SymbolTable,
    error::ErrorCollector,
    resolution::{
        DependencyGraphNode,
        dependency_analysis::{
            DependencyGraph, build_func_param_graph, build_struct_and_protocol_graph,
            build_var_graph,
        },
    },
    symbol_path,
};

pub fn build_graph(ec: &mut ErrorCollector, symbol_table: &SymbolTable) -> Option<DependencyGraph> {
    let mut graph = DependencyGraph::new();

    // Output variables MUST have type annotations therefore we don't need to resolve their types.
    for stmt in &symbol_table.inputs {
        if let ParserStatementKind::Input {
            name,
            value_type: _,
            def_val,
            attrs: _,
        } = &stmt.1.kind
        {
            // Combine variable name to create a new path for the child type
            let var_path = symbol_path![SymbolPathComponent::InputVar(name.to_string())];
            build_var_graph(ec, &mut graph, symbol_table, &var_path, def_val);
            graph.add_node(DependencyGraphNode::new(var_path));
        }
    }

    for stmt in &symbol_table.outputs {
        if let ParserStatementKind::Output {
            name,
            value_type: _,
            def_val: _,
        } = &stmt.1.kind
        {
            // Combine variable name to create a new path for the child type
            let var_path = symbol_path![SymbolPathComponent::OutputVar(name.to_string())];
            graph.add_node(DependencyGraphNode::new(var_path));
        }
    }

    for stmt in &symbol_table.states {
        if let ParserStatementKind::State { vars } = &stmt.1.kind {
            for var in vars {
                // Combine variable name to create a new path for the child type
                let var_path = symbol_path![SymbolPathComponent::StateVar(var.name.to_string())];
                build_var_graph(ec, &mut graph, symbol_table, &var_path, &var.def_val);
                graph.add_node(DependencyGraphNode::new(var_path));
            }
        }
    }

    for stmt in &symbol_table.funcs {
        if let ParserStatementKind::FuncDecl {
            required_by: _,
            name,
            params,
            return_type: _,
            body: _,
        } = &stmt.1.kind
        {
            // Combine variable name to create a new path for the function
            let func_path = symbol_path![SymbolPathComponent::Func(name.to_string())];
            build_func_param_graph(ec, &mut graph, symbol_table, &func_path, params);
            graph.add_node(DependencyGraphNode::new(func_path));
        }
    }

    for stmt in &symbol_table.type_defs {
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
                if let Some(decl_stmt) = symbol_table.get_type_def(name) {
                    let child_symbol_table = &decl_stmt.1;
                    let child_type_path =
                        symbol_path![SymbolPathComponent::TypeDef(name.to_string())];

                    build_struct_and_protocol_graph(
                        ec,
                        &mut graph,
                        &child_type_path,
                        symbol_table,
                        child_symbol_table,
                    );
                }
            }

            _ => (),
        }
    }

    let infix_funcs = &symbol_table
        .infix_funcs
        .values()
        .flatten()
        .collect::<Vec<&&ParserStatement>>();

    for stmt in infix_funcs {
        if let ParserStatementKind::OperatorFunc {
            op_type: _,
            symbol,
            params,
            return_type: _,
            body: _,
        } = &stmt.kind
        {
            // Combine variable name to create a new path for the function
            let func_path = symbol_path![SymbolPathComponent::InfixFunc(symbol.to_string())];
            build_func_param_graph(ec, &mut graph, symbol_table, &func_path, params);
            graph.add_node(DependencyGraphNode::new(func_path));
        }
    }

    let prefix_funcs = &symbol_table
        .prefix_funcs
        .values()
        .flatten()
        .collect::<Vec<&&ParserStatement>>();

    for stmt in prefix_funcs {
        if let ParserStatementKind::OperatorFunc {
            op_type: _,
            symbol,
            params,
            return_type: _,
            body: _,
        } = &stmt.kind
        {
            // Combine variable name to create a new path for the function
            let func_path = symbol_path![SymbolPathComponent::PrefixFunc(symbol.to_string())];
            build_func_param_graph(ec, &mut graph, symbol_table, &func_path, params);
            graph.add_node(DependencyGraphNode::new(func_path));
        }
    }

    for stmt in &symbol_table.infix_defines {
        if let ParserStatementKind::InfixDefine { symbol, .. } = &stmt.1.kind {
            let def_path = symbol_path![SymbolPathComponent::InfixDef(symbol.to_string())];
            graph.add_node(DependencyGraphNode::new(def_path));
        }
    }

    for stmt in &symbol_table.prefix_defines {
        if let ParserStatementKind::PrefixDefine { symbol } = &stmt.1.kind {
            let def_path = symbol_path![SymbolPathComponent::PrefixDef(symbol.to_string())];
            graph.add_node(DependencyGraphNode::new(def_path));
        }
    }

    Some(graph)
}
