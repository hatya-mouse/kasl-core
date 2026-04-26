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
    ast_nodes::{
        Range, Statement,
        type_registry::{PrimitiveType, ResolvedType},
    },
    error::Ph,
    parser::{ExprToken, ParserScopeStmt},
    semantic_analysis::{
        expr_engine::resolve_expr, stmt_builder::BlockStmtBuilder, utils::get_constant_int,
    },
};

impl BlockStmtBuilder<'_> {
    pub fn build_loop_stmt(
        &mut self,
        count: &[ExprToken],
        body: &[ParserScopeStmt],
        decl_range: Range,
    ) -> Option<Statement> {
        // Parse the count
        let count_expr = resolve_expr(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            self.scope_id,
            self.namespace_id,
            count,
        )?;

        // Check if the count has integer type
        if !matches!(
            count_expr.value_type,
            ResolvedType::Primitive(PrimitiveType::Int)
        ) {
            self.ec.non_integer_for_loop_count(
                count_expr.range,
                Ph::ExprEngine,
                self.prog_ctx
                    .type_registry
                    .format_type(&count_expr.value_type),
            );
            return None;
        }

        // Verify the count expression and get the integer loop count
        let Some(loop_count) = get_constant_int(&self.prog_ctx.scope_registry, &count_expr) else {
            self.ec
                .non_constant_for_loop_count(count_expr.range, Ph::ExprEngine);
            return None;
        };

        // Build the body and return the new loop statement
        let loop_block = self.build_scope_block(body, self.scope_id, decl_range);
        Some(Statement::Loop {
            count: loop_count,
            body: loop_block,
        })
    }
}
