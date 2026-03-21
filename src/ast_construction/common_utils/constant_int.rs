use crate::{Expr, ExprKind, ScopeRegistry, scope_manager::VariableKind};

pub(crate) fn get_constant_int(scope_registry: &ScopeRegistry, expr: &Expr) -> Option<u32> {
    match &expr.kind {
        ExprKind::IntLiteral(value) => Some(*value),
        ExprKind::Identifier(id) => {
            // Check if the variable is a constant
            if let Some(scope_var) = scope_registry.get_var(id)
                && matches!(
                    scope_var.var_kind,
                    VariableKind::GlobalConst | VariableKind::LocalConst
                )
                && let Some(def_val) = &scope_var.def_val
            {
                get_constant_int(scope_registry, def_val)
            } else {
                None
            }
        }
        _ => None,
    }
}
