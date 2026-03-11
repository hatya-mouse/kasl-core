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
    ExprToken, ExprTokenKind, Range, error::Ph, expr_engine::LValueResolver, symbol_table::LValue,
    type_registry::ResolvedType,
};

impl LValueResolver<'_> {
    pub fn resolve_l_value(&mut self, tokens: &[ExprToken]) -> Option<LValue> {
        // Resolve the first token
        let mut token_iter = tokens.iter();
        let first = token_iter.next()?;
        let mut lvalue = self.resolve_single(first)?;

        // Check the remaining tokens for field access
        while let Some(dot_token) = token_iter.next() {
            if dot_token.kind != ExprTokenKind::Dot {
                // If the token after an identifier is not a dot, throw an error
                self.ec.invalid_l_value(dot_token.range, Ph::ExprEngine);
            }

            let Some(identifier_token) = token_iter.next() else {
                // If the token after a dot doesn't exist, throw an error
                self.ec
                    .non_member_token_after_dot(dot_token.range, Ph::ExprEngine);
                return None;
            };

            let identifier_name = match &identifier_token.kind {
                ExprTokenKind::Identifier(name) => name,
                _ => {
                    self.ec
                        .non_member_token_after_dot(identifier_token.range, Ph::ExprEngine);
                    return None;
                }
            };

            // Get the StructDecl
            let ResolvedType::Struct(struct_id) = lvalue.value_type else {
                self.ec
                    .member_access_on_primitive(identifier_token.range, Ph::ExprEngine);
                return None;
            };
            let struct_decl = self.type_registry.get_struct(&struct_id)?;

            // Retrieve the field index
            let struct_field_index = match struct_decl.get_field_index(identifier_name) {
                Some(index) => index,
                None => {
                    self.ec.member_field_not_found(
                        identifier_token.range,
                        Ph::ExprEngine,
                        struct_decl.name.clone(),
                        identifier_name.clone(),
                    );
                    return None;
                }
            };

            // Get the offset
            let struct_offset = struct_decl.get_offset_by_index(struct_field_index)?;
            // Get the StructField
            let struct_field = struct_decl.get_field_by_index(struct_field_index)?;

            lvalue.offset += struct_offset;
            lvalue.value_type = struct_field.value_type;
        }

        Some(lvalue)
    }

    pub fn resolve_single(&mut self, token: &ExprToken) -> Option<LValue> {
        match &token.kind {
            ExprTokenKind::Identifier(name) => self.resolve_identifier(name, token.range),
            _ => {
                self.ec.invalid_l_value(token.range, Ph::ExprEngine);
                None
            }
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

        // Create and return a LValue
        Some(LValue {
            var_id: *var_id,
            offset: 0,
            value_type: var.value_type,
            is_field: false,
        })
    }
}
