use crate::{
    Expr, ExprKind, ExprToken,
    error::Ph,
    expr_engine::{LValueResolver, resolve_expr},
    scope_manager::VariableKind,
    symbol_table::{LValue, LValueKind},
};

impl LValueResolver<'_> {
    pub fn resolve_l_value(&mut self, tokens: &[ExprToken]) -> Option<LValue> {
        // Resolve the expression to LValue
        let expr = resolve_expr(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            self.current_scope,
            self.current_namespace,
            tokens,
        )?;

        // Recursively validate the l-value expression
        self.expr_to_l_value(expr)
    }

    fn expr_to_l_value(&mut self, expr: Expr) -> Option<LValue> {
        match expr.kind {
            ExprKind::Identifier(var_id) => {
                // Check if the LValue is a writable variable
                if let Some(target_var) = self.prog_ctx.scope_registry.get_var(&var_id)
                    && matches!(
                        target_var.var_kind,
                        VariableKind::Input { .. }
                            | VariableKind::GlobalConst
                            | VariableKind::LocalConst
                            | VariableKind::FuncParam
                    )
                {
                    self.ec.immutable_assignment(
                        expr.range,
                        Ph::StatementBuilding,
                        &target_var.name,
                    );
                    return None;
                }

                Some(LValue::new(LValueKind::Identifier(var_id), expr.value_type))
            }
            ExprKind::StructField { lhs, offset } => {
                let l_vaule_lhs = self.expr_to_l_value(*lhs)?;
                Some(LValue::new(
                    LValueKind::StructField {
                        lhs: Box::new(l_vaule_lhs),
                        offset,
                    },
                    expr.value_type,
                ))
            }
            ExprKind::Subscript { lhs, index } => {
                let l_vaule_lhs = self.expr_to_l_value(*lhs)?;
                Some(LValue::new(
                    LValueKind::Subscript {
                        lhs: Box::new(l_vaule_lhs),
                        index,
                    },
                    expr.value_type,
                ))
            }
            _ => {
                self.ec.invalid_l_value(expr.range, Ph::ExprEngine);
                None
            }
        }
    }
}
