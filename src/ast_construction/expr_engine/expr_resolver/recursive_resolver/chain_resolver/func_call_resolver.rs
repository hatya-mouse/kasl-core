//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{
    ast::{
        Expr, ExprKind, NameSpaceID, Range, symbol_table::NoTypeFuncCallArg,
        type_registry::ResolvedType,
    },
    ast_construction::expr_engine::ExpressionResolver,
    error::Ph,
};

impl ExpressionResolver<'_> {
    pub fn resolve_func_call(
        &mut self,
        namespace_id: NameSpaceID,
        name: &str,
        no_type_args: &[NoTypeFuncCallArg],
        range: Range,
    ) -> Option<Expr> {
        if let Some(func_id) = self
            .prog_ctx
            .func_ctx
            .get_global_func_id(namespace_id, name)
        {
            // Get a reference to the function
            let func = self.prog_ctx.func_ctx.get_func(&func_id)?;
            let func_params = func.params.clone();
            let func_scope_id = func.block.scope_id;
            let func_return_type = func.return_type;

            let args = self.resolve_func_call_args(&func_params, None, no_type_args, range)?;

            // Add a function call edge to the scope graph
            // This is used to detect recursion
            self.comp_data
                .scope_graph
                .add_edge(self.current_scope, func_scope_id);

            Some(Expr::new(
                ExprKind::FuncCall { id: func_id, args },
                func_return_type,
                range,
            ))
        } else if let Some(struct_id) = self
            .prog_ctx
            .type_registry
            .get_struct_id(namespace_id, name)
        {
            // If the function does not exist, check if the type with the same name exists
            // Ensure that the function doesn't have any arguments
            if !no_type_args.is_empty() {
                self.ec.arg_for_struct_init(range, Ph::ExprEngine);
            }

            Some(Expr::new(
                ExprKind::StructInit { id: struct_id },
                ResolvedType::Struct(struct_id),
                range,
            ))
        } else {
            self.ec.func_not_found(range, Ph::ExprEngine, name);
            None
        }
    }
}
