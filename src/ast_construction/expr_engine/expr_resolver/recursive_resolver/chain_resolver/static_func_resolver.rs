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
    Expr, ExprKind, Range, StructID,
    error::Ph,
    expr_engine::ExpressionResolver,
    symbol_table::{FunctionType, UnresolvedChainElement},
};

impl ExpressionResolver<'_> {
    pub fn resolve_static_func_call(
        &mut self,
        struct_id: StructID,
        element: &UnresolvedChainElement,
        range: Range,
    ) -> Option<Expr> {
        match element {
            UnresolvedChainElement::Identifier { .. } => {
                self.ec.static_var_access(range, Ph::ExprEngine);
                None
            }
            UnresolvedChainElement::FuncCall {
                name,
                args: no_type_args,
            } => {
                // Get the function ID by name
                let Some(func_id) = self.prog_ctx.func_ctx.get_member_func_id(&struct_id, name)
                else {
                    let struct_decl = self.prog_ctx.type_registry.get_struct(&struct_id)?;
                    self.ec
                        .member_func_not_found(range, Ph::ExprEngine, &struct_decl.name, name);
                    return None;
                };

                // Get the function by ID
                let func = self.prog_ctx.func_ctx.get_func(&func_id)?;

                // Throw an error if the function is not static
                if func.func_type != FunctionType::Static {
                    self.ec
                        .static_call_of_instance_func(range, Ph::ExprEngine, &func.name);
                    return None;
                }

                // Capture the function informations
                let func_params = func.params.clone();
                let func_scope = func.block.scope_id;
                let func_return_type = func.return_type;

                // Resolve the arguments
                let args = self.resolve_func_call_args(
                    &func_params,
                    None,
                    no_type_args,
                    &func_scope,
                    range,
                )?;

                // Construct the expression
                Some(Expr::new(
                    ExprKind::StaticFuncCall { id: func_id, args },
                    func_return_type,
                    range,
                ))
            }
        }
    }
}
