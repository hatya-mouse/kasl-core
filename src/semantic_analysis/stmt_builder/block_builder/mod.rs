//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

/// Builds an Assign statement.
mod assign;
/// Builds a block statement.
mod block_stmt;
/// Builds an Expression statement.
mod expr_stmt;
/// Builds an If statement.
mod if_stmt;
/// Builds a LocalVar and LocalConst statements which declare local variables and constants.
mod local_decl;
/// Builds a ScopeVar from the given information and registers it in the scope registry.
mod local_var_registrar;
/// Builds a Loop statement.
mod loop_stmt;
/// Builds a Return statement.
mod return_stmt;
/// Builds a Block which contains ScopeID from a list of statements.
mod scope_block;

use crate::{
    ast_nodes::{
        CompilationData, NameSpaceID, ScopeID, Statement, compilation_data::ProgramContext,
        flow_graph::FlowGraphBuilder, type_registry::ResolvedType,
    },
    builtin::BuiltinRegistry,
    error::ErrorCollector,
    parser::{ParserScopeStmt, ParserScopeStmtKind},
};

/// Builds a statements from raw parser statements.
/// Should not be reused across multiple functions.
pub struct BlockStmtBuilder<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: &'a mut CompilationData,
    builtin_registry: &'a BuiltinRegistry,

    flow_graph_builder: &'a mut FlowGraphBuilder,

    scope_id: ScopeID,
    namespace_id: NameSpaceID,
    expected_return_type: ResolvedType,
}

impl<'a> BlockStmtBuilder<'a> {
    #[allow(
        clippy::too_many_arguments,
        reason = "Compiler data such as error collector, program context, compilation data, builtin registry, and flow graph builder must be passed to the function.
        Also scope_id, namespace_id and expected_return_type is crucial for this function so they cannot be reduced."
    )]
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_data: &'a mut CompilationData,
        builtin_registry: &'a BuiltinRegistry,
        flow_graph_builder: &'a mut FlowGraphBuilder,
        scope_id: ScopeID,
        namespace_id: NameSpaceID,
        expected_return_type: ResolvedType,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            comp_data,
            builtin_registry,
            flow_graph_builder,
            scope_id,
            namespace_id,
            expected_return_type,
        }
    }

    pub fn build_stmt(&mut self, stmt: &ParserScopeStmt) -> Option<Statement> {
        match &stmt.kind {
            ParserScopeStmtKind::Block { statements } => {
                self.build_block_stmt(statements, stmt.range)
            }
            ParserScopeStmtKind::LocalVar {
                name,
                value_type,
                def_val,
            } => self.build_local_var(name, value_type, def_val, stmt.range),
            ParserScopeStmtKind::LocalConst {
                name,
                value_type,
                def_val,
            } => self.build_local_const(name, value_type, def_val, stmt.range),
            ParserScopeStmtKind::Assign { target, value } => {
                self.build_assign(target, value, stmt.range)
            }
            ParserScopeStmtKind::Expression { expr } => self.build_expr_stmt(expr),
            ParserScopeStmtKind::If {
                main,
                else_ifs,
                else_body,
                else_range,
            } => self.build_if_stmt(main, else_ifs, else_body.as_ref(), *else_range),
            ParserScopeStmtKind::Return { value } => {
                self.build_return_stmt(value.as_ref(), stmt.range)
            }
            ParserScopeStmtKind::Loop { count, body } => {
                self.build_loop_stmt(count, body, stmt.range)
            }
        }
    }

    // Marks the name as used in the namespace.

    pub fn mark_name_used(&mut self, name: &str) {
        // Mark the name as used in the namespace
        self.prog_ctx
            .scope_registry
            .mark_name_used(&self.scope_id, name);
    }

    pub fn is_name_used(&self, name: &str) -> bool {
        self.prog_ctx
            .scope_registry
            .is_name_used(&self.scope_id, name)
    }
}
