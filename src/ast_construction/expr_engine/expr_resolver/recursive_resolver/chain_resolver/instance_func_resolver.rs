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
    Expr, ExprKind, Range, error::Ph, expr_engine::ExpressionResolver,
    symbol_table::NoTypeFuncCallArg, type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_instance_func_call(
        &mut self,
        lhs: Expr,
        name: &str,
        no_type_args: &Vec<NoTypeFuncCallArg>,
        range: Range,
    ) -> Option<Expr> {
        // Get the field from the type of the lhs expression
        match lhs.value_type {
            ResolvedType::Primitive(_) => {
                self.ec.member_access_on_primitive(
                    range,
                    Ph::ExprEngine,
                    lhs.value_type.to_string(),
                );
                return None;
            }
            ResolvedType::Struct(struct_id) => {
                // Get the function
                let Some(member_func_id) =
                    self.prog_ctx.func_ctx.get_member_func_id(&struct_id, &name)
                else {
                    let struct_decl = self.prog_ctx.type_registry.get_struct(&struct_id)?;
                    self.ec
                        .member_func_not_found(range, Ph::ExprEngine, &struct_decl.name, name);
                    return None;
                };
                let member_func = self.prog_ctx.func_ctx.get_func(&member_func_id)?;
                // Resolve the arguments
                let args =
                    self.resolve_func_call_args(&member_func.params, &no_type_args, range)?;

                // Return the struct field expression
                return Some(Expr::new(
                    ExprKind::InstanceFuncCall {
                        lhs: Box::new(lhs),
                        id: member_func_id,
                        args,
                    },
                    member_func.return_type,
                    range,
                ));
            }
        }
    }
}
