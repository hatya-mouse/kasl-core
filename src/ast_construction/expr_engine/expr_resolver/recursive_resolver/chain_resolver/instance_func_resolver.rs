use crate::{
    Expr, ExprKind, Range, error::Ph, expr_engine::ExpressionResolver,
    symbol_table::NoTypeFuncCallArg, type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_instance_func_call(
        &mut self,
        lhs: Expr,
        name: &str,
        no_type_args: &[NoTypeFuncCallArg],
        range: Range,
    ) -> Option<Expr> {
        // Get the field from the type of the lhs expression
        match lhs.value_type {
            ResolvedType::Primitive(_) => {
                self.ec.member_access_on_primitive(
                    range,
                    Ph::ExprEngine,
                    self.prog_ctx.type_registry.format_type(&lhs.value_type),
                );
                None
            }
            ResolvedType::Array(_) => {
                self.ec.member_access_on_array(
                    range,
                    Ph::ExprEngine,
                    self.prog_ctx.type_registry.format_type(&lhs.value_type),
                );
                None
            }
            ResolvedType::Struct(struct_id) => {
                // Get the function
                let Some(member_func_id) =
                    self.prog_ctx.func_ctx.get_member_func_id(&struct_id, name)
                else {
                    let struct_decl = self.prog_ctx.type_registry.get_struct(&struct_id)?;
                    self.ec
                        .member_func_not_found(range, Ph::ExprEngine, &struct_decl.name, name);
                    return None;
                };
                let member_func = self.prog_ctx.func_ctx.get_func(&member_func_id)?;
                let member_func_params = member_func.params.clone();
                let return_type = member_func.return_type;

                // Resolve the arguments
                let args = self.resolve_func_call_args(
                    &member_func_params,
                    Some(lhs),
                    no_type_args,
                    range,
                )?;

                // Return the struct field expression
                Some(Expr::new(
                    ExprKind::InstanceFuncCall {
                        id: member_func_id,
                        args,
                    },
                    return_type,
                    range,
                ))
            }
        }
    }
}
