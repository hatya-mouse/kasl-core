use crate::{
    Expr, ExprKind, Range,
    error::Ph,
    expr_engine::ExpressionResolver,
    symbol_table::UnresolvedExpr,
    type_registry::{PrimitiveType, ResolvedType},
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
