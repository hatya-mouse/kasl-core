use crate::{
    Expr, ExprToken, Range, ScopeVar, common_utils::resolve_type, error::Ph,
    expr_engine::resolve_expr, global_decl_collection::GlobalDeclCollector,
    parser_ast::ParserTypeName, scope_manager::VariableKind,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_def_val_global(
        &mut self,
        type_annotation: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        decl_range: Range,
    ) -> Option<Expr> {
        // Resolve the default value expression
        let global_scope_id = self
            .prog_ctx
            .scope_registry
            .get_global_scope_id(&self.current_namespace);
        let resolved_def_val = resolve_expr(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            global_scope_id,
            self.current_namespace,
            def_val,
        )?;

        if let Some(type_annotation) = type_annotation {
            // Resolve the type annotation if provided
            let resolved_type_annotation = match resolve_type(self.prog_ctx, type_annotation) {
                Some(ty) => ty,
                None => {
                    self.ec.type_not_found(
                        decl_range,
                        Ph::GlobalDeclCollection,
                        type_annotation.to_string(),
                    );
                    return None;
                }
            };

            // If the type annotation provided by the user does not match the default value type throw an error
            if resolved_def_val.value_type != resolved_type_annotation {
                self.ec.type_annotation_mismatch(
                    decl_range,
                    Ph::GlobalDeclCollection,
                    self.prog_ctx
                        .type_registry
                        .format_type(&resolved_type_annotation),
                    self.prog_ctx
                        .type_registry
                        .format_type(&resolved_def_val.value_type),
                );
                return None;
            }
        }

        Some(resolved_def_val)
    }

    pub fn register_var_globally(
        &mut self,
        name: &str,
        type_annotation: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        var_kind: VariableKind,
        decl_range: Range,
    ) {
        // Check if the name is already in use in this scope
        if self.is_name_used(name) {
            self.ec
                .duplicate_name(decl_range, Ph::StatementBuilding, name);
            return;
        }

        // Resolve the default value expression
        let Some(resolved_def_val) =
            self.resolve_def_val_global(type_annotation, def_val, decl_range)
        else {
            return;
        };

        // Get the global scope ID
        let global_scope_id = self
            .prog_ctx
            .scope_registry
            .get_global_scope_id(&self.current_namespace);

        // Register the variable in the global scope
        let var = ScopeVar {
            name: name.to_string(),
            value_type: resolved_def_val.value_type,
            def_val: Some(resolved_def_val),
            range: decl_range,
            var_kind,
        };
        self.prog_ctx
            .scope_registry
            .register_var(var, name.to_string(), &global_scope_id);

        // Mark the variable name as used in the namespace
        self.mark_name_used(name);
    }
}
