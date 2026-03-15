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
    Expr, ExprKind, Range, error::Ph, expr_engine::ExpressionResolver, symbol_table::MemberAccess,
    type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_static_func_call(
        &mut self,
        base_type: &ResolvedType,
        access: MemberAccess,
        range: Range,
    ) -> Option<Expr<ResolvedType>> {
        // Get the id of the struct
        let ResolvedType::Struct(struct_id) = base_type else {
            self.ec
                .member_access_on_primitive(range, Ph::ExprEngine, base_type.to_string());
            return None;
        };

        // Assume static member access is a func call
        match access {
            MemberAccess::Access { .. } => {
                self.ec.static_var_access(range, Ph::ExprEngine);
                None
            }
            MemberAccess::FuncCall {
                name, no_type_args, ..
            } => {
                // Get the function ID by name
                let Some(func_id) = self
                    .namespace
                    .func_ctx
                    .get_member_func_by_name(struct_id, &name)
                else {
                    let struct_decl = self.namespace.type_registry.get_struct(struct_id)?;
                    self.ec.member_func_not_found(
                        range,
                        Ph::ExprEngine,
                        struct_decl.name.clone(),
                        name,
                    );
                    return None;
                };

                // Get the function by ID
                let func = self.namespace.func_ctx.get_func(&func_id)?;

                // Throw an error if the function is static
                if func.is_static {
                    self.ec
                        .static_func_call_on_instance(range, Ph::ExprEngine, &func.name);
                    return None;
                }

                // Resolve the arguments
                let args = self.resolve_func_call_args(&func.params, &no_type_args, range)?;

                // Construct the expression
                Some(Expr::new(
                    ExprKind::StaticFuncCall {
                        name,
                        id: func_id,
                        args,
                    },
                    func.return_type,
                    range,
                ))
            }
        }
    }
}
