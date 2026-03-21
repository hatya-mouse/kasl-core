use crate::{
    Expr, ExprKind, Range,
    error::Ph,
    expr_engine::ExpressionResolver,
    symbol_table::{NoTypeFuncCallArg, UnresolvedChainElement},
    type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_builtin_func_call(
        &mut self,
        element: &UnresolvedChainElement,
        builtin_range: Range,
    ) -> Option<Expr> {
        match element {
            UnresolvedChainElement::Identifier { range, .. } => {
                self.ec.builtin_var_access(*range, Ph::ExprEngine);
                None
            }
            UnresolvedChainElement::FuncCall {
                name,
                args: no_type_args,
                range,
            } => {
                // Get the function ID by name
                let Some(func_id) = self.builtin_registry.get_id_by_name(name) else {
                    self.ec.builtin_func_not_found(*range, Ph::ExprEngine, name);
                    return None;
                };

                // Get the function by ID
                let func = self.builtin_registry.get_func_by_id(func_id)?;

                // Resolve the arguments
                let args = self.resolve_builtin_args(&func.params, no_type_args, *range)?;

                // Construct the expression
                Some(Expr::new(
                    ExprKind::BuiltinFuncCall { id: *func_id, args },
                    func.return_type,
                    Range::n(builtin_range.start, range.end),
                ))
            }
        }
    }

    fn resolve_builtin_args(
        &mut self,
        expected_params: &[ResolvedType],
        no_type_args: &[NoTypeFuncCallArg],
        range: Range,
    ) -> Option<Vec<Expr>> {
        let mut args = Vec::new();
        for (expected_type, no_type_arg) in expected_params.iter().zip(no_type_args) {
            let resolved_arg = self.resolve_recursively(no_type_arg.value.clone())?;
            // Check if the type of the argument matches the expected type
            if &resolved_arg.value_type != expected_type {
                self.ec.builtin_arg_type_mismatch(
                    range,
                    Ph::ExprEngine,
                    self.prog_ctx.type_registry.format_type(expected_type),
                    self.prog_ctx
                        .type_registry
                        .format_type(&resolved_arg.value_type),
                );
            }

            args.push(resolved_arg);
        }
        Some(args)
    }
}
