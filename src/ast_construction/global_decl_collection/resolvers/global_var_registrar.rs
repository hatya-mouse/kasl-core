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
    type_registry::ResolvedType,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_def_val_global(
        &mut self,
        type_annotation: &Option<SymbolPath>,
        def_val: &Vec<ExprToken>,
        decl_range: Range,
    ) -> Option<Expr<ResolvedType>> {
        // Resolve the default value expression
        let global_scope_id = self.scope_registry.get_global_scope_id();
        let resolved_def_val = resolve_expr(
            self.ec,
            self.op_ctx,
            self.func_ctx,
            self.scope_registry,
            self.type_registry,
            global_scope_id,
            def_val.clone(),
        )?;

        // Resolve the type annotation if provided
        let resolved_type_annotation = type_annotation
            .as_ref()
            .and_then(|path| self.type_registry.resolve_type_path(path))?;

        // If the type annotation provided by the user does not match the default value type throw an error
        if resolved_def_val.value_type != resolved_type_annotation {
            self.ec.type_annotation_mismatch(
                decl_range,
                Ph::GlobalDeclCollection,
                resolved_type_annotation.to_string(),
                resolved_def_val.value_type.to_string(),
            );
            return None;
        }

        Some(resolved_def_val)
    }

    pub fn register_var_globally(
        &mut self,
        name: &str,
        type_annotation: &Option<SymbolPath>,
        def_val: &Vec<ExprToken>,
        var_kind: VariableKind,
        decl_range: Range,
    ) {
        // Resolve the default value expression
        let Some(resolved_def_val) =
            self.resolve_def_val_global(type_annotation, def_val, decl_range)
        else {
            return;
        };

        // Register the variable in the global scope
        let var = ScopeVar {
            name: name.to_string(),
            def_val: resolved_def_val,
            range: decl_range,
            var_kind,
        };
        let variable_id = self.name_space.generate_variable_id();
        let global_scope_id = self.scope_registry.get_global_scope_id();
        self.scope_registry
            .register_var(var, name.to_string(), variable_id, global_scope_id);
    }
}
