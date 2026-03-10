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
    ExprToken, ExprTokenKind, ParserMemberAccess, Range, error::Ph, expr_engine::LValueResolver,
    symbol_table::LValue, type_registry::ResolvedType,
};

impl LValueResolver<'_> {
    pub fn resolve_recursively(&mut self, raw_expr: &ExprToken) -> Option<LValue> {
        match &raw_expr.kind {
            ExprTokenKind::Identifier(name) => self.resolve_identifier(name, raw_expr.range),
            ExprTokenKind::Chain { lhs, member } => self.resolve_chain(lhs, member, raw_expr.range),
            _ => None,
        }
    }

    pub fn resolve_identifier(&mut self, name: &str, range: Range) -> Option<LValue> {
        // Look up the variable ID in the current scope
        let Some(var_id) = self.scope_registry.lookup_var(self.current_scope, name) else {
            self.ec.var_not_found(range, Ph::ExprEngine, name);
            return None;
        };

        // Get the variable's type
        let var = self.scope_registry.get_var_by_id(var_id)?;
        let value_type = var.def_val.value_type;

        // Create and return a LValue
        Some(LValue {
            var_id: *var_id,
            offset: 0,
            value_type,
            is_field: false,
        })
    }

    pub fn resolve_chain(
        &mut self,
        lhs: &ExprToken,
        member: &ParserMemberAccess,
        range: Range,
    ) -> Option<LValue> {
        match member {
            ParserMemberAccess::Access(name) => {
                // Resolve the LHS
                let lhs = self.resolve_recursively(lhs)?;

                // Get the StructDecl
                let ResolvedType::Struct(struct_id) = lhs.value_type else {
                    self.ec.member_access_on_primitive(range, Ph::ExprEngine);
                    return None;
                };
                let struct_decl = self.type_registry.get_struct(&struct_id)?;

                // Retrieve the field index
                let struct_field_index = match struct_decl.get_field_index(name) {
                    Some(index) => index,
                    None => {
                        self.ec.member_field_not_found(
                            range,
                            Ph::ExprEngine,
                            struct_decl.name.clone(),
                            name.clone(),
                        );
                        return None;
                    }
                };

                // Get the offset
                let struct_offset = struct_decl.get_offset_by_index(struct_field_index)?;
                let offset = lhs.offset + struct_offset;
                // Get the value type of the StructField
                let struct_field = struct_decl.get_field_by_index(struct_field_index)?;

                Some(LValue {
                    var_id: lhs.var_id,
                    offset,
                    value_type: struct_field.value_type,
                    is_field: true,
                })
            }
            ParserMemberAccess::FuncCall { .. } => {
                self.ec.func_call_in_l_value(range, Ph::ExprEngine);
                None
            }
        }
    }
}
