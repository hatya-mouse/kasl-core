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
    Expr, ExprKind, Range, error::Ph, expr_engine::ExpressionResolver, type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_field_access(&mut self, lhs: Expr, name: &str, range: Range) -> Option<Expr> {
        // Get the field from the type of the lhs expression
        match lhs.value_type {
            ResolvedType::Primitive(_) => {
                self.ec.member_access_on_primitive(
                    range,
                    Ph::ExprEngine,
                    self.prog_ctx.type_registry.format_type(&lhs.value_type),
                );
                None
            }
            ResolvedType::Array(_) => {
                self.ec.member_access_on_array(
                    range,
                    Ph::ExprEngine,
                    self.prog_ctx.type_registry.format_type(&lhs.value_type),
                );
                None
            }
            ResolvedType::Struct(struct_id) => {
                let struct_decl = self.prog_ctx.type_registry.get_struct(&struct_id)?;
                // Get the field from the struct declaration
                let Some(field_index) = struct_decl.get_field_index(name) else {
                    self.ec
                        .member_field_not_found(range, Ph::ExprEngine, &struct_decl.name, name);
                    return None;
                };

                // Get the offset of the field
                let field_type = struct_decl.fields[field_index].value_type;
                let field_offset = struct_decl.field_offsets[field_index];
                // Return the struct field expression
                Some(Expr::new(
                    ExprKind::StructField {
                        lhs: Box::new(lhs),
                        offset: field_offset,
                    },
                    field_type,
                    range,
                ))
            }
        }
    }
}
