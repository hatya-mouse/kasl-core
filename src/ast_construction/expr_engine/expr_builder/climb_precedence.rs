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
    ast::{
        OperatorAssociativity,
        symbol_table::{UnresolvedExpr, UnresolvedExprKind},
    },
    ast_construction::expr_engine::ExpressionBuilder,
    error::Ph,
    parser::{ExprToken, ExprTokenKind},
};
use std::{iter::Peekable, slice::Iter};

impl ExpressionBuilder<'_> {
    pub fn climb_precedence(
        &mut self,
        tokens: &mut Peekable<Iter<ExprToken>>,
        min_prec: u32,
    ) -> Option<UnresolvedExpr> {
        // Get the left-hand side expression
        let mut lhs = self.parse_lhs_chain(tokens)?;

        while let Some(op_token) = tokens.peek() {
            // Get the range of the operator token
            let op_range = op_token.range;

            let ExprTokenKind::Operator(op_symbol) = &op_token.kind else {
                break;
            };

            if let Some(op_props) = self.op_ctx.get_postfix_props(op_symbol) {
                // Break if the operator precedence is less than the minimum precedence
                if op_props.precedence < min_prec {
                    break;
                }

                lhs = UnresolvedExpr::new(
                    UnresolvedExprKind::PostfixOp {
                        symbol: op_symbol.clone(),
                        operand: Box::new(lhs),
                    },
                    op_range,
                );
                tokens.next();
            } else {
                // If the operator is not a postfix operator, assume it's infix
                let Some(op_props) = self.op_ctx.get_infix_props(op_symbol) else {
                    // If the both infix and postfix operators are not found, emit an error
                    self.ec.infix_or_postfix_op_not_defined(
                        op_token.range,
                        Ph::ExprEngine,
                        op_symbol,
                    );
                    break;
                };

                if op_props.precedence < min_prec {
                    // Break if the operator precedence is less than the minimum precedence
                    break;
                }

                if op_props.precedence == min_prec
                    && op_props.associativity == OperatorAssociativity::None
                {
                    // Throw an error if the operator is not associative but consecutively used
                    self.ec
                        .op_not_associative(op_token.range, Ph::ExprEngine, op_symbol);
                    return None;
                }

                // Calculate the next precedence based on associativity
                let next_prec = match op_props.associativity {
                    OperatorAssociativity::Left => op_props.precedence + 1,
                    OperatorAssociativity::Right => op_props.precedence,
                    OperatorAssociativity::None => op_props.precedence + 1,
                };

                // Then consume the operator token
                tokens.next();

                let rhs = self.climb_precedence(tokens, next_prec)?;
                lhs = UnresolvedExpr::new(
                    UnresolvedExprKind::InfixOp {
                        symbol: op_symbol.clone(),
                        lhs_expr: Box::new(lhs),
                        rhs_expr: Box::new(rhs),
                    },
                    op_range,
                );
            }
        }

        Some(lhs)
    }
}
