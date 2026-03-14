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
    Range, error::Ph, expr_engine::ExpressionResolver, symbol_table::MemberAccess,
    type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_member_access(
        &mut self,
        lhs_type: &ResolvedType,
        access: MemberAccess,
        range: Range,
    ) -> Option<(MemberAccess, ResolvedType)> {
        match lhs_type {
            // If the LHS is a primitive type, the member access is invalid
            ResolvedType::Primitive(primitive_type) => {
                self.ec.member_access_on_primitive(
                    range,
                    Ph::ExprEngine,
                    primitive_type.to_string(),
                );
                None
            }

            // If the LHS is a struct type, get the offset of the field
            ResolvedType::Struct(struct_id) => match access {
                MemberAccess::Access { name, .. } => {
                    self.resolve_field_access(struct_id, name, range)
                }

                MemberAccess::FuncCall {
                    name, no_type_args, ..
                } => self.resolve_member_func_call(struct_id, name, no_type_args, range),
            },
        }
    }
}
