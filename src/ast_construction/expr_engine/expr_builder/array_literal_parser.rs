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
    ast::symbol_table::{UnresolvedExpr, UnresolvedExprKind},
    ast_construction::expr_engine::ExpressionBuilder,
    parser::{ExprToken, ExprTokenKind},
};

impl ExpressionBuilder<'_> {
    pub fn parse_array_literal(&mut self, token: &ExprToken) -> Option<UnresolvedExpr> {
        // Collect until the matching bracket
        let ExprTokenKind::Bracketed(bracket_items) = &token.kind else {
            return None;
        };

        // Parse the array literal
        if let Some(semi_pos) = bracket_items
            .iter()
            .position(|t| matches!(t.kind, ExprTokenKind::Semicolon))
        {
            let value_tokens = &bracket_items[..semi_pos];
            let count_tokens = &bracket_items[semi_pos + 1..];
            let value = self.build(value_tokens)?;
            let count = self.build(count_tokens)?;
            Some(UnresolvedExpr::new(
                UnresolvedExprKind::ArraySpread {
                    value: Box::new(value),
                    count: Box::new(count),
                },
                token.range,
            ))
        } else {
            let items = self.split_by_comma(bracket_items);
            let exprs: Vec<UnresolvedExpr> = items
                .iter()
                .map(|item| self.build(item))
                .collect::<Option<Vec<_>>>()?;
            Some(UnresolvedExpr::new(
                UnresolvedExprKind::ArrayList(exprs),
                token.range,
            ))
        }
    }

    fn split_by_comma(&self, tokens: &[ExprToken]) -> Vec<Vec<ExprToken>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        for token in tokens {
            match &token.kind {
                ExprTokenKind::Comma => {
                    result.push(current.clone());
                    current = Vec::new();
                }
                _ => {
                    current.push(token.clone());
                }
            }
        }
        if !current.is_empty() {
            result.push(current);
        }
        result
    }
}
