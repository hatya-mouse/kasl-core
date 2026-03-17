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
        let first_token = match token_iter.next() {
            Some(token) => token,
            None => {
                // If the expression does not have any tokens, it is invalid
                self.ec.invalid_l_value(tokens[0].range, Ph::ExprEngine);
                return None;
            }
        };

        let mut l_value = if let ExprTokenKind::Identifier(name) = &first_token.kind {
            match self.resolve_identifier(target_scope, name, first_token.range) {
                Some(lv) => lv,
                None => {
                    self.ec.invalid_l_value(first_token.range, Ph::ExprEngine);
                    return None;
                }
            }
        } else {
            self.ec.invalid_l_value(first_token.range, Ph::ExprEngine);
            return None;
        };

        while let Some(token) = token_iter.next() {
            if token.kind != ExprTokenKind::Dot {
                self.ec.invalid_l_value(token.range, Ph::ExprEngine);
                break;
            }

            let Some(next_token) = token_iter.next() else {
                self.ec.expr_ends_with_dot(token.range, Ph::ExprEngine);
                break;
            };

            if let ExprTokenKind::Identifier(name) = &next_token.kind {
                l_value = match self.resolve_field_access(l_value, name, next_token.range) {
                    Some(lv) => lv,
                    None => {
                        self.ec.invalid_l_value(next_token.range, Ph::ExprEngine);
                        return None;
                    }
                };
            } else {
                // If the token is not an identifier, throw an error and return None
                self.ec.invalid_l_value(next_token.range, Ph::ExprEngine);
                return None;
            }
        }

        Some(l_value)
    }
}
