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
    ParserOperatorType, ParserTopLevelStmtKind, PrimitiveType, Program, Range, SymbolTable,
    error::{ErrorCollector, Phase},
    resolution::{
        TypeResolveCtx,
        dependency_analysis::{build_graph, sort_graph},
    },
};

/// Infer the types of symbols (input, output, state, var, and function parameters) in the program.
pub fn resolve_types(ec: &mut ErrorCollector, program: &mut Program, symbol_table: &SymbolTable) {
    // Register primitive types
    program.add_primitive_type(PrimitiveType::Int);
    program.add_primitive_type(PrimitiveType::Float);
    program.add_primitive_type(PrimitiveType::Bool);

    // Build the type dependency graph
    let graph = match build_graph(ec, symbol_table) {
        Some(graph) => graph,
        None => return,
    };

    // Then sort symbols based on the dependency graph
    let sorted_list = match sort_graph(ec, symbol_table, &graph) {
        Some(sorted_list) => sorted_list,
        None => return,
    };

    // Get references from the sorted symbol paths
    let mut statements = Vec::new();

    for symbol_id in &sorted_list {
        // Get the statements in order
        if let Some(current_stmt) = symbol_table.get_statement_by_id(symbol_id) {
            statements.push(current_stmt);
        } else {
            ec.comp_bug(
                Range::zero(),
                Phase::GraphConstruction,
                &format!(
                    "SymbolPath(s) in the dependency graph must be valid: {:?}",
                    symbol_id
                ),
            );
        }
    }

    // Create a TypeResolveCtx instance
    let mut ctx = TypeResolveCtx::new(ec, program, symbol_table);

    // Infer the type of each symbol in the sorted order
    for (symbol_id, current_stmt) in sorted_list.iter().zip(statements) {
        // Check if the symbol has already got a type annotation
        // If not, infer the type
        match &current_stmt.kind {
            ParserTopLevelStmtKind::Input {
                name,
                value_type,
                def_val,
                attrs,
            } => ctx.resolve_input(
                name,
                value_type.as_ref(),
                def_val,
                attrs,
                current_stmt.range,
            ),

            ParserTopLevelStmtKind::Output {
                name,
                value_type,
                def_val,
            } => ctx.resolve_output(name, value_type.as_ref(), def_val, current_stmt.range),

            ParserTopLevelStmtKind::StateVar {
                name,
                value_type,
                def_val,
            } => ctx.resolve_state(name, value_type.as_ref(), def_val, current_stmt.range),

            ParserTopLevelStmtKind::ScopeVar {
                name,
                value_type,
                def_val,
            } => ctx.resolve_var(
                name,
                symbol_id,
                value_type.as_ref(),
                def_val,
                current_stmt.range,
            ),

            ParserTopLevelStmtKind::FuncDecl {
                is_static,
                name,
                params,
                return_type,
                body: _,
            } => ctx.resolve_func(
                *is_static,
                name,
                symbol_id,
                params,
                return_type.as_ref(),
                current_stmt.range,
            ),

            ParserTopLevelStmtKind::OperatorFunc {
                op_type,
                symbol,
                params,
                return_type,
                body: _,
            } => match op_type {
                ParserOperatorType::Infix => {
                    ctx.resolve_infix_func(symbol, params, return_type, current_stmt.range)
                }
                ParserOperatorType::Prefix => {
                    ctx.resolve_prefix_operator(symbol, params, return_type, current_stmt.range)
                }
            },

            ParserTopLevelStmtKind::InfixDefine {
                symbol,
                infix_properties,
            } => ctx.register_infix_define(symbol, infix_properties.clone()),

            ParserTopLevelStmtKind::StructDecl { name, .. } => ctx.register_struct(symbol_id, name),
        }
    }
}
