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
    Expr, ExprKind, Range, StructID, error::Ph, expr_engine::ExpressionResolver,
    symbol_table::MemberAccess, type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_field_access(
        &mut self,
        lhs: Expr<ResolvedType>,
        struct_id: &StructID,
        name: String,
        range: Range,
    ) -> Option<Expr<ResolvedType>> {
        // Get the struct declaration
        let struct_decl = self.namespace.type_registry.get_struct(struct_id)?;

        // Get the index of the field by its name
        let Some(field_index) = struct_decl.get_field_index(&name) else {
            self.ec.member_field_not_found(
                range,
                Ph::ExprEngine,
                struct_decl.name.clone(),
                name.clone(),
            );
            return None;
        };

        // Then get the offset of the field
        let field = struct_decl.get_field_by_index(field_index)?;
        let offset = struct_decl.get_offset_by_index(field_index)?;

        // Construct the member access and the expression
        let resolved_access = MemberAccess::Access {
            name,
            offset: Some(offset),
        };
        Some(Expr::new(
            ExprKind::Chain {
                lhs: Box::new(lhs),
                access: resolved_access,
            },
            field.value_type,
            range,
        ))
    }
}
