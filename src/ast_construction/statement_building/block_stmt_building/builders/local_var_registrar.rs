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
    Expr, ExprToken, Range, ScopeVar, VariableID, error::Ph, expr_engine::resolve_expr,
    parser_ast::ParserTypeName, scope_manager::VariableKind, statement_building::BlockStmtBuilder,
    type_resolver::resolve_type,
};

impl BlockStmtBuilder<'_> {
    fn resolve_def_val(
        &mut self,
        def_val: &[ExprToken],
        value_type: &Option<ParserTypeName>,
        stmt_range: Range,
    ) -> Option<Expr> {
        let resolved_def_val = resolve_expr(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            self.scope_id,
            self.namespace_id,
            def_val,
        )?;

        // Resolve the type annotation if provided
        if let Some(type_annotation) = value_type {
            let resolved_type_annotation = match resolve_type(self.prog_ctx, type_annotation) {
                Some(ty) => ty,
                None => {
                    self.ec.type_not_found(
                        stmt_range,
                        Ph::StatementBuilding,
                        type_annotation.to_string(),
                    );
                    return None;
                }
            };

            // Check if the resolved value type matches the type annotation
            if resolved_type_annotation != resolved_def_val.value_type {
                self.ec.type_annotation_mismatch(
                    stmt_range,
                    Ph::StatementBuilding,
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

    pub fn build_and_register_scope_var(
        &mut self,
        name: &str,
        value_type: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        stmt_range: Range,
        var_kind: VariableKind,
    ) -> Option<VariableID> {
        // Resolve the default value expression
        let resolved_def_val = self.resolve_def_val(def_val, value_type, stmt_range)?;

        // Create a ScopeVar
        let scope_var = ScopeVar {
            name: name.to_string(),
            value_type: resolved_def_val.value_type,
            def_val: Some(resolved_def_val),
            range: stmt_range,
            var_kind,
        };

        // Check if the name is already in use in this scope
        if self.is_name_used(name) {
            self.ec
                .duplicate_name(stmt_range, Ph::StatementBuilding, name);
            return None;
        }

        // Register the variable in the scope
        let var_id =
            self.prog_ctx
                .scope_registry
                .register_var(scope_var, name.to_string(), &self.scope_id);

        // Mark the variable name as used in the namespace
        self.mark_name_used(name);

        Some(var_id)
    }
}
