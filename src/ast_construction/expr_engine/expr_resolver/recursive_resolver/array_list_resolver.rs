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
    Expr, ExprKind, Range, error::Ph, expr_engine::ExpressionResolver,
    symbol_table::UnresolvedExpr, type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_array_list(
        &mut self,
        items: Vec<UnresolvedExpr>,
        expr_range: Range,
    ) -> Option<Expr> {
        // If the array is empty throw an error
        if items.is_empty() {
            self.ec.empty_array_literal(expr_range, Ph::ExprEngine);
            return None;
        }

        // Resolve the items each
        let mut resolved_items = Vec::new();
        for item in items {
            let resolved = self.resolve_recursively(item)?;
            resolved_items.push(resolved);
        }

        // Check if the all items have the same typr
        let first_type = resolved_items.first()?.value_type;
        for item in &resolved_items {
            if item.value_type != first_type {
                let first_name = self.prog_ctx.type_registry.format_type(&first_type);
                let item_name = self.prog_ctx.type_registry.format_type(&item.value_type);
                self.ec
                    .array_item_type_mismatch(expr_range, Ph::ExprEngine, first_name, item_name);
            }
        }

        // Register or get an array declaration
        let array_id = self
            .prog_ctx
            .type_registry
            .register_or_get_array(first_type, resolved_items.len() as u32);

        Some(Expr::new(
            ExprKind::ArrayList(resolved_items),
            ResolvedType::Array(array_id),
            expr_range,
        ))
    }
}
