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

use crate::{
    ast_nodes::{Range, Statement},
    error::Ph,
    parser::ExprToken,
    semantic_analysis::{expr_engine::resolve_expr, stmt_builder::BlockStmtBuilder},
};

impl BlockStmtBuilder<'_> {
    pub fn build_return_stmt(
        &mut self,
        value: Option<&Vec<ExprToken>>,
        decl_range: Range,
    ) -> Option<Statement> {
        // The current scope has a return statement
        self.flow_graph_builder.current_has_return();

        if self.expected_return_type.is_void() {
            if value.is_some() {
                // If the function doesn't require a return value but it's provided, throw and error
                self.ec
                    .return_value_for_no_return_func(decl_range, Ph::StatementBuilding);
                return None;
            }

            return Some(Statement::Return { value: None });
        }

        let Some(value) = value else {
            self.ec.return_without_value_for_return_func(
                decl_range,
                Ph::StatementBuilding,
                self.prog_ctx
                    .type_registry
                    .format_type(&self.expected_return_type),
            );
            return None;
        };

        // Resolve the expression
        let resolved_value = resolve_expr(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            self.scope_id,
            self.namespace_id,
            value,
        )?;

        // Check if the return type matches the expected return type
        // If the self.expected_return_type is None, resolved_value should be None as well
        if resolved_value.value_type != self.expected_return_type {
            self.ec.return_type_mismatch(
                decl_range,
                Ph::StatementBuilding,
                self.prog_ctx
                    .type_registry
                    .format_type(&self.expected_return_type),
                self.prog_ctx
                    .type_registry
                    .format_type(&resolved_value.value_type),
            );
            return None;
        }

        Some(Statement::Return {
            value: Some(resolved_value),
        })
    }
}
