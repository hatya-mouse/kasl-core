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
    Expr, ExprToken, Range, ScopeVar, SymbolPath, error::Ph, expr_engine::resolve_expr,
    global_decl_collection::GlobalDeclCollector, scope_manager::VariableKind,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_def_val_global(
        &mut self,
        type_annotation: &Option<SymbolPath>,
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

        // Resolve the type annotation if provided
        if let Some(path) = type_annotation {
            let (namespace_id, type_name) = self
                .prog_ctx
                .namespace_registry
                .resolve_namespace_from_path(path.clone());
            let resolved_type_annotation = match self
                .prog_ctx
                .type_registry
                .resolve_type(namespace_id, &type_name.to_string())
            {
                Some(ty) => ty,
                None => {
                    self.ec
                        .type_not_found(decl_range, Ph::GlobalDeclCollection, path.to_string());
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
        type_annotation: &Option<SymbolPath>,
        def_val: &[ExprToken],
        var_kind: VariableKind,
        decl_range: Range,
    ) {
        // Check if the name is already in use in this scope
        if self.is_name_used(name) {
            self.ec
                .duplicate_name(decl_range, Ph::StatementCollection, name);
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
