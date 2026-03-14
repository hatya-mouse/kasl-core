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
    Range, StructID,
    error::Ph,
    expr_engine::ExpressionResolver,
    symbol_table::{MemberAccess, NoTypeFuncCallArg},
    type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_member_func_call(
        &mut self,
        struct_id: &StructID,
        name: String,
        no_type_args: Vec<NoTypeFuncCallArg>,
        range: Range,
    ) -> Option<(MemberAccess, ResolvedType)> {
        // Get the function ID by name
        let Some(func_id) = self
            .comp_state
            .func_ctx
            .get_member_func_by_name(struct_id, &name)
        else {
            let struct_decl = self.comp_state.type_registry.get_struct(struct_id)?;
            self.ec
                .member_func_not_found(range, Ph::ExprEngine, struct_decl.name.clone(), name);
            return None;
        };

        // Get the function by ID
        let func = self.comp_state.func_ctx.get_func(&func_id)?;

        // Resolve the arguments
        let args = self.resolve_func_call_args(&func.params, &no_type_args, range)?;
        Some((
            MemberAccess::FuncCall {
                name,
                no_type_args,
                args: Some(args),
                id: Some(func_id),
            },
            func.return_type,
        ))
    }
}
