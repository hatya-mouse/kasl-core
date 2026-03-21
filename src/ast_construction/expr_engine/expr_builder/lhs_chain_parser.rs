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

use crate::Range;
use crate::expr_engine::ExpressionBuilder;
use crate::symbol_table::{UnresolvedChainElement, UnresolvedExpr, UnresolvedExprKind};
use crate::{ExprToken, ExprTokenKind, error::Ph};
use std::{iter::Peekable, slice::Iter};

impl ExpressionBuilder<'_> {
    pub fn parse_lhs_chain(
        &mut self,
        tokens: &mut Peekable<Iter<ExprToken>>,
    ) -> Option<UnresolvedExpr> {
        let first = tokens.next()?;
        let mut expr = self.parse_lhs_single(first, tokens)?;
        expr = self.resolve_chain(expr, tokens)?;

        Some(expr)
    }

    fn resolve_chain(
        &mut self,
        expr: UnresolvedExpr,
        tokens: &mut Peekable<Iter<ExprToken>>,
    ) -> Option<UnresolvedExpr> {
        let mut result = expr;
        loop {
            match tokens.peek().map(|t| &t.kind) {
                Some(ExprTokenKind::Dot) => {
                    result = self.resolve_member_access(result, tokens)?;
                }
                Some(ExprTokenKind::BracketOpen) => {
                    result = self.resolve_subscript(result, tokens)?;
                }
                _ => break,
            }
        }
        Some(result)
    }

    fn resolve_member_access(
        &mut self,
        expr: UnresolvedExpr,
        tokens: &mut Peekable<Iter<ExprToken>>,
    ) -> Option<UnresolvedExpr> {
        let mut chain_elements = Vec::new();

        // If the next token is dot, resolve it as a member access
        while let Some(ExprTokenKind::Dot) = tokens.peek().map(|token| &token.kind) {
            // Consume the dot token
            // The next token is confirmed to be a dot so it can be safely unwrapped
            let dot_token = tokens.next().unwrap();
            // Get the next token
            let Some(next_token) = tokens.next() else {
                self.ec.expr_ends_with_dot(dot_token.range, Ph::ExprEngine);
                return None;
            };

            let chain_element = match &next_token.kind {
                ExprTokenKind::Identifier(name) => UnresolvedChainElement::Identifier {
                    name: name.clone(),
                    range: next_token.range,
                },
                ExprTokenKind::FuncCall { name, args } => UnresolvedChainElement::FuncCall {
                    name: name.clone(),
                    args: self.parse_func_args(args)?,
                    range: next_token.range,
                },
                _ => {
                    self.ec
                        .non_member_token_after_dot(next_token.range, Ph::ExprEngine);
                    return None;
                }
            };
            chain_elements.push(chain_element);
        }

        // If the chain elements are not empty, construct the chain expression
        if !chain_elements.is_empty() {
            if let UnresolvedExprKind::Chain { lhs, elements } = expr.kind {
                let joined_elements = [elements.clone(), chain_elements].concat();
                Some(UnresolvedExpr::new(
                    UnresolvedExprKind::Chain {
                        lhs: lhs.clone(),
                        elements: joined_elements,
                    },
                    expr.range,
                ))
            } else {
                let expr_range = expr.range;
                Some(UnresolvedExpr::new(
                    UnresolvedExprKind::Chain {
                        lhs: Some(Box::new(expr)),
                        elements: chain_elements,
                    },
                    expr_range,
                ))
            }
        } else {
            Some(expr)
        }
    }

    fn resolve_subscript(
        &mut self,
        expr: UnresolvedExpr,
        tokens: &mut Peekable<Iter<ExprToken>>,
    ) -> Option<UnresolvedExpr> {
        let mut result_expr = expr;

        while let Some(ExprTokenKind::BracketOpen) = tokens.peek().map(|token| &token.kind) {
            // Consume the bracket open token
            let bracket_open_range = tokens.next().unwrap().range;

            // Collect until the matching bracket
            let (index_tokens, close_bracket_end) =
                self.collect_bracket_contents(bracket_open_range, tokens)?;

            // Build the unresolved token for the index expression
            let index_expr = self.build(&index_tokens)?;

            // Create a new subscript token ans return it
            result_expr = UnresolvedExpr::new(
                UnresolvedExprKind::Subscript {
                    lhs: Box::new(result_expr),
                    index: Box::new(index_expr),
                },
                Range::n(bracket_open_range.start, close_bracket_end),
            );
        }

        Some(result_expr)
    }
}
