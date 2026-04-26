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
        Expr, ExprKind, Range,
        symbol_table::UnresolvedExpr,
        type_registry::{PrimitiveType, ResolvedType},
    },
    error::Ph,
    semantic_analysis::{expr_engine::ExpressionResolver, utils::get_constant_int},
};

impl ExpressionResolver<'_> {
    pub fn resolve_array_spread(
        &mut self,
        value: UnresolvedExpr,
        count: UnresolvedExpr,
        expr_range: Range,
    ) -> Option<Expr> {
        // Resolve the value
        let resolved_value = self.resolve_recursively(value)?;
        // Resolve the count
        let resolved_count = self.resolve_recursively(count)?;

        // Check if the count has integer type
        if !matches!(
            resolved_count.value_type,
            ResolvedType::Primitive(PrimitiveType::Int)
        ) {
            self.ec.non_integer_for_array_count(
                resolved_count.range,
                Ph::ExprEngine,
                self.prog_ctx
                    .type_registry
                    .format_type(&resolved_count.value_type),
            );
            return None;
        }

        // Check if the count is a constant and get the value
        if let Some(count_value) = get_constant_int(&self.prog_ctx.scope_registry, &resolved_count)
        {
            // Create new array type of get the existing one
            let array_id = self
                .prog_ctx
                .type_registry
                .register_or_get_array(resolved_value.value_type, count_value);
            let array_type = ResolvedType::Array(array_id);

            Some(Expr::new(
                ExprKind::ArraySpread {
                    value: Box::new(resolved_value),
                    count: count_value,
                },
                array_type,
                expr_range,
            ))
        } else {
            self.ec
                .non_constant_for_array_count(resolved_count.range, Ph::ExprEngine);
            None
        }
    }
}
