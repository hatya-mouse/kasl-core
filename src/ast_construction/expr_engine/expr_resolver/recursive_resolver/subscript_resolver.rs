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
    ast::{
        Expr, ExprKind, Range,
        symbol_table::UnresolvedExpr,
        type_registry::{PrimitiveType, ResolvedType},
    },
    ast_construction::expr_engine::ExpressionResolver,
    error::Ph,
};

impl ExpressionResolver<'_> {
    pub fn resolve_subscript(
        &mut self,
        lhs: UnresolvedExpr,
        index: UnresolvedExpr,
        expr_range: Range,
    ) -> Option<Expr> {
        // Resolve the LHS expression
        let resolved_lhs = self.resolve_recursively(lhs)?;

        // Check the LHS type
        if let ResolvedType::Array(array_id) = resolved_lhs.value_type {
            let resolved_index = self.resolve_recursively(index)?;

            // Check if the index value is of integer
            if matches!(
                resolved_index.value_type,
                ResolvedType::Primitive(PrimitiveType::Int)
            ) {
                // Get the array declaration to get the item type
                let array_decl = self.prog_ctx.type_registry.get_array_decl(&array_id)?;

                // Create a new subscript expression
                Some(Expr::new(
                    ExprKind::Subscript {
                        lhs: Box::new(resolved_lhs),
                        index: Box::new(resolved_index),
                    },
                    *array_decl.item_type(),
                    expr_range,
                ))
            } else {
                self.ec.non_integer_in_subscript(
                    expr_range,
                    Ph::ExprEngine,
                    self.prog_ctx
                        .type_registry
                        .format_type(&resolved_index.value_type),
                );
                None
            }
        } else {
            self.ec.subscript_on_non_array(
                expr_range,
                Ph::ExprEngine,
                self.prog_ctx
                    .type_registry
                    .format_type(&resolved_lhs.value_type),
            );
            None
        }
    }
}
