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
    Range, error::Ph, expr_engine::LValueResolver, symbol_table::LValue,
    type_registry::ResolvedType,
};

impl LValueResolver<'_> {
    pub fn resolve_field_access(
        &mut self,
        last_l_value: LValue,
        name: &str,
        range: Range,
    ) -> Option<LValue> {
        // Get the field from the type of the last l value
        match last_l_value.value_type {
            ResolvedType::Primitive(_) => {
                self.ec.member_access_on_primitive(
                    range,
                    Ph::ExprEngine,
                    last_l_value.value_type.to_string(),
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
                println!("type: {:?}", field_type);
                let field_offset = struct_decl.field_offsets[field_index];
                // Return the resolved l value
                Some(LValue {
                    var_id: last_l_value.var_id,
                    offset: last_l_value.offset + field_offset,
                    value_type: field_type,
                    is_field: true,
                })
            }
        }
    }
}
