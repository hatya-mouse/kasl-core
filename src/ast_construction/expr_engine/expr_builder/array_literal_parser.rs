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
    ExprToken, ExprTokenKind, Range,
    error::Ph,
    expr_engine::ExpressionBuilder,
    symbol_table::{UnresolvedExpr, UnresolvedExprKind},
};
use std::{iter::Peekable, slice::Iter};

impl ExpressionBuilder<'_> {
    pub fn collect_bracket_contents(
        &mut self,
        bracket_open_range: Range,
        tokens: &mut Peekable<Iter<ExprToken>>,
    ) -> Option<(Vec<ExprToken>, usize)> {
        let mut index_tokens: Vec<ExprToken> = Vec::new();
        let close_bracket_end: usize;
        let mut depth = 1;
        loop {
            let Some(token) = tokens.next() else {
                self.ec
                    .unmatched_bracket(bracket_open_range, Ph::ExprEngine);
                return None;
            };
            match &token.kind {
                ExprTokenKind::BracketOpen => {
                    depth += 1;
                    index_tokens.push(token.clone());
                }
                ExprTokenKind::BracketClose => {
                    depth -= 1;
                    if depth == 0 {
                        close_bracket_end = token.range.end;
                        break;
                    }
                    index_tokens.push(token.clone());
                }
                _ => {
                    index_tokens.push(token.clone());
                }
            }
        }
        Some((index_tokens, close_bracket_end))
    }

    pub fn parse_array_literal(
        &mut self,
        token: &ExprToken,
        rest: &mut Peekable<Iter<ExprToken>>,
    ) -> Option<UnresolvedExpr> {
        // Collect until the matching bracket
        let (bracket_items, close_bracket_end) =
            self.collect_bracket_contents(token.range, rest)?;

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
            let items = self.split_by_comma(&bracket_items);
            let exprs: Vec<UnresolvedExpr> = items
                .iter()
                .map(|item| self.build(item))
                .collect::<Option<Vec<_>>>()?;
            Some(UnresolvedExpr::new(
                UnresolvedExprKind::ArrayList(exprs),
                Range::n(token.range.start, close_bracket_end),
            ))
        }
    }

    fn split_by_comma(&self, tokens: &[ExprToken]) -> Vec<Vec<ExprToken>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        let mut depth = 0;
        for token in tokens {
            match &token.kind {
                ExprTokenKind::BracketOpen => {
                    depth += 1;
                    current.push(token.clone());
                }
                ExprTokenKind::BracketClose => {
                    depth -= 1;
                    current.push(token.clone());
                }
                ExprTokenKind::Comma if depth == 0 => {
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
