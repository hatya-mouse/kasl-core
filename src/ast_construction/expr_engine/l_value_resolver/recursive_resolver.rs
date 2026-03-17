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
    ExprToken, ExprTokenKind, error::Ph, expr_engine::LValueResolver, symbol_table::LValue,
};

impl LValueResolver<'_> {
    pub fn resolve_l_value(&mut self, tokens: &[ExprToken]) -> Option<LValue> {
        let mut token_iter = tokens.iter().peekable();

        // Get the target scope from the tokens
        let target_scope = self.resolve_namespace_scope(&mut token_iter);

        // Resolve the identifier
        let mut l_value: Option<LValue> = None;
        while let Some(token) = token_iter.peek() {
            if let ExprTokenKind::Identifier(name) = &token.kind {
                if let Some(last_l_value) = l_value {
                    l_value = self.resolve_field_access(last_l_value, name, token.range);
                } else {
                    l_value = self.resolve_identifier(target_scope, name, token.range);
                }
            } else {
                // If the token is not an identifier, throw an error and return None
                self.ec.invalid_l_value(token.range, Ph::ExprEngine);
                return None;
            }
        }
        l_value
    }
}
