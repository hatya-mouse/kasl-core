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
    Expr, ExprKind, ExprToken, ExprTokenKind, OperatorAssociativity, ParserFuncCallArg,
    ParserMemberAccess,
    error::Ph,
    expr_engine::ExpressionBuilder,
    symbol_table::{MemberAccess, NoTypeFuncCallArg},
};
use std::{iter::Peekable, slice::Iter};

impl ExpressionBuilder<'_> {
    pub fn climb_precedence(
        &mut self,
        tokens: &mut Peekable<Iter<ExprToken>>,
        min_prec: u32,
    ) -> Option<Expr<()>> {
        // Get the left-hand side expression
        let mut lhs = self.parse_lhs(tokens)?;

        while let Some(op_token) = tokens.peek() {
            // Get the range of the operator token
            let op_range = op_token.range;

            let op_symbol = match &op_token.kind {
                ExprTokenKind::Operator(symbol) => symbol.clone(),
                _ => break,
            };

            if let Some(op_props) = self.op_ctx.get_postfix_props(&op_symbol) {
                // Break if the operator precedence is less than the minimum precedence
                if op_props.precedence < min_prec {
                    break;
                }

                lhs = Expr::new(
                    ExprKind::PostfixOp {
                        symbol: op_symbol,
                        operator: None,
                        operand: Box::new(lhs),
                    },
                    (),
                    op_range,
                );
                tokens.next();
            } else {
                // If the operator is not a postfix operator, assume it's infix
                let op_props = match self.op_ctx.get_infix_props(&op_symbol) {
                    Some(op_props) => op_props,
                    None => {
                        // If the both infix and postfix operators are not found, emit an error
                        self.ec.infix_or_postfix_op_not_found(
                            op_token.range,
                            Ph::ExprEngine,
                            &op_symbol,
                        );
                        break;
                    }
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
                        .op_not_associative(op_token.range, Ph::ExprEngine, &op_symbol);
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
                lhs = Expr::new(
                    ExprKind::InfixOp {
                        symbol: op_symbol,
                        operator: None,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    },
                    (),
                    op_range,
                );
            }
        }

        Some(lhs)
    }

    fn parse_lhs(&mut self, tokens: &mut Peekable<Iter<ExprToken>>) -> Option<Expr<()>> {
        let first = tokens.next()?;
        self.parse_lhs_single(first, tokens)
    }

    fn parse_lhs_single(
        &mut self,
        token: &ExprToken,
        rest: &mut Peekable<Iter<ExprToken>>,
    ) -> Option<Expr<()>> {
        match &token.kind {
            ExprTokenKind::Operator(symbol) => {
                let prefix_prec = match self.op_ctx.get_prefix_props(&symbol) {
                    Some(op_props) => op_props.precedence,
                    None => {
                        self.ec
                            .prefix_op_not_found(token.range, Ph::ExprEngine, &symbol);
                        return None;
                    }
                };
                let operand = self.climb_precedence(rest, prefix_prec)?;
                Some(Expr::new(
                    ExprKind::PrefixOp {
                        symbol: symbol.clone(),
                        operator: None,
                        operand: Box::new(operand),
                    },
                    (),
                    token.range,
                ))
            }

            ExprTokenKind::IntLiteral(value) => {
                Some(Expr::new(ExprKind::IntLiteral(*value), (), token.range))
            }
            ExprTokenKind::FloatLiteral(value) => {
                Some(Expr::new(ExprKind::FloatLiteral(*value), (), token.range))
            }
            ExprTokenKind::BoolLiteral(value) => {
                Some(Expr::new(ExprKind::BoolLiteral(*value), (), token.range))
            }

            ExprTokenKind::Identifier(name) => Some(Expr::new(
                ExprKind::Identifier {
                    name: name.clone(),
                    id: None,
                },
                (),
                token.range,
            )),

            ExprTokenKind::FuncCall { name, args } => Some(Expr::new(
                ExprKind::FuncCall {
                    name: name.clone(),
                    id: None,
                    no_type_args: self.parse_func_args(args.clone())?,
                    args: None,
                },
                (),
                token.range,
            )),

            ExprTokenKind::Chain { lhs, member } => {
                let lhs_expr = self.parse_lhs_single(lhs, &mut [].iter().peekable())?;
                let member_access = match member {
                    ParserMemberAccess::Access(name) => MemberAccess::Access {
                        name: name.clone(),
                        offset: None,
                    },
                    ParserMemberAccess::FuncCall { name, args } => MemberAccess::FuncCall {
                        name: name.clone(),
                        id: None,
                        no_type_args: self.parse_func_args(args.clone())?,
                        args: None,
                    },
                };
                Some(Expr::new(
                    ExprKind::Chain {
                        lhs: Box::new(lhs_expr),
                        access: member_access,
                    },
                    (),
                    token.range,
                ))
            }

            ExprTokenKind::ResolvedExpr(expr) => Some(expr.clone()),

            ExprTokenKind::Parenthesized(_) => {
                self.ec.comp_bug(
                    token.range,
                    Ph::GlobalDeclCollection,
                    "Parenthesized expression should have already been parsed by build() function.",
                );
                None
            }
        }
    }

    fn parse_func_args(&mut self, args: Vec<ParserFuncCallArg>) -> Option<Vec<NoTypeFuncCallArg>> {
        let mut parsed_args = Vec::new();
        for arg in args {
            let arg_expr = self.build(arg.value)?;
            parsed_args.push(NoTypeFuncCallArg {
                label: arg.label,
                value: arg_expr,
                range: arg.range,
            });
        }
        Some(parsed_args)
    }
}
