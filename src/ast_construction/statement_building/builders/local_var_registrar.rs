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
    Expr, ExprToken, Range, ScopeID, ScopeVar, SymbolPath, VariableID, error::Ph,
    expr_engine::resolve_expr, scope_manager::VariableKind, statement_building::StatementBuilder,
    type_registry::ResolvedType,
};

impl StatementBuilder<'_> {
    fn resolve_def_val(
        &mut self,
        def_val: &[ExprToken],
        value_type: &Option<SymbolPath>,
        current_scope_id: ScopeID,
        decl_range: Range,
    ) -> Option<Expr<ResolvedType>> {
        let resolved_def_val = resolve_expr(
            self.ec,
            self.op_ctx,
            self.func_ctx,
            self.scope_registry,
            self.type_registry,
            current_scope_id,
            def_val,
        )?;

        // Resolve the type annotation if provided
        if let Some(type_annotation) = value_type {
            let Some(resolved_type_annotation) =
                self.type_registry.resolve_type_path(type_annotation)
            else {
                self.ec.type_not_found(
                    decl_range,
                    Ph::StatementCollection,
                    type_annotation.to_string(),
                );
                return None;
            };

            // Check if the resolved value type matches the type annotation
            if resolved_type_annotation != resolved_def_val.value_type {
                self.ec.type_annotation_mismatch(
                    decl_range,
                    Ph::StatementCollection,
                    resolved_type_annotation.to_string(),
                    resolved_def_val.value_type.to_string(),
                );
                return None;
            }
        }

        Some(resolved_def_val)
    }

    pub fn build_and_register_scope_var(
        &mut self,
        name: &str,
        value_type: &Option<SymbolPath>,
        def_val: &[ExprToken],
        current_scope_id: ScopeID,
        decl_range: Range,
        var_kind: VariableKind,
    ) -> Option<VariableID> {
        // Resolve the default value expression
        let resolved_def_val =
            self.resolve_def_val(def_val, value_type, current_scope_id, decl_range)?;

        // Create a ScopeVar
        let scope_var = ScopeVar {
            name: name.to_string(),
            def_val: resolved_def_val,
            range: decl_range,
            var_kind,
        };

        // Check if the name is already in use in this scope
        if self.scope_registry.contains_var(current_scope_id, name) {
            self.ec
                .duplicate_var_name(decl_range, Ph::StatementCollection, name);
            return None;
        }

        // Register the variable in the scope
        let var_id = self.name_space.generate_variable_id();
        self.scope_registry
            .register_var(scope_var, name.to_string(), var_id, current_scope_id);

        Some(var_id)
    }
}
