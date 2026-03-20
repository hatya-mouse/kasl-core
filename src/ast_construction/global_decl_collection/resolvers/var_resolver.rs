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
    ExprToken, InputAttribute, ParserInputAttribute, Range, expr_engine::resolve_expr,
    global_decl_collection::GlobalDeclCollector, parser_ast::ParserTypeName,
    scope_manager::VariableKind,
};

impl GlobalDeclCollector<'_> {
    pub fn resolve_input(
        &mut self,
        name: &str,
        value_type: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        attrs: &[ParserInputAttribute],
        decl_range: Range,
    ) {
        // Resolve the attributes
        let Some(resolved_attrs) = self.resolve_attrs(attrs) else {
            return;
        };

        // Register the input variable
        self.register_var_globally(
            name,
            value_type,
            def_val,
            VariableKind::Input {
                attrs: resolved_attrs,
            },
            decl_range,
        );
    }

    pub fn resolve_output(
        &mut self,
        name: &str,
        value_type: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        decl_range: Range,
    ) {
        self.register_var_globally(name, value_type, def_val, VariableKind::Output, decl_range);
    }

    pub fn resolve_state_var(
        &mut self,
        name: &str,
        value_type: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        decl_range: Range,
    ) {
        self.register_var_globally(name, value_type, def_val, VariableKind::State, decl_range);
    }

    pub fn resolve_global_const(
        &mut self,
        name: &str,
        value_type: &Option<ParserTypeName>,
        def_val: &[ExprToken],
        decl_range: Range,
    ) {
        self.register_var_globally(
            name,
            value_type,
            def_val,
            VariableKind::GlobalConst,
            decl_range,
        );
    }

    fn resolve_attrs(&mut self, attrs: &[ParserInputAttribute]) -> Option<Vec<InputAttribute>> {
        let mut resolved_attrs = Vec::new();
        let global_scope_id = self
            .prog_ctx
            .scope_registry
            .get_global_scope_id(&self.current_namespace);

        // Resolve each attribute's arguments and construct InputAttribute
        for attr in attrs {
            let mut resolved_args = Vec::new();
            for arg in &attr.args {
                // Resolve the expression of the argument
                let resolved_arg = resolve_expr(
                    self.ec,
                    self.prog_ctx,
                    self.comp_data,
                    self.builtin_registry,
                    global_scope_id,
                    self.current_namespace,
                    arg,
                )?;
                resolved_args.push(resolved_arg);
            }

            resolved_attrs.push(InputAttribute {
                name: attr.name.clone(),
                args: resolved_args,
                range: attr.range,
            });
        }
        Some(resolved_attrs)
    }
}
