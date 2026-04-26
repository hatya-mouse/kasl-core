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

use crate::ast_nodes::symbol_table::{
    NoTypeFuncCallArg, UnresolvedChainElement, UnresolvedExpr, UnresolvedExprKind,
};
use crate::error::Ph;
use crate::parser::{ExprToken, ExprTokenKind, ParserFuncCallArg};
use crate::semantic_analysis::expr_engine::ExpressionBuilder;
use std::{iter::Peekable, slice::Iter};

impl ExpressionBuilder<'_> {
    pub fn parse_lhs_single(
        &mut self,
        token: &ExprToken,
        rest: &mut Peekable<Iter<ExprToken>>,
    ) -> Option<UnresolvedExpr> {
        match &token.kind {
            ExprTokenKind::Operator(symbol) => {
                let prefix_prec = match self.op_ctx.get_prefix_props(symbol) {
                    Some(op_props) => op_props.precedence,
                    None => {
                        self.ec
                            .prefix_op_not_defined(token.range, Ph::ExprEngine, symbol);
                        return None;
                    }
                };
                let operand = self.climb_precedence(rest, prefix_prec)?;
                Some(UnresolvedExpr::new(
                    UnresolvedExprKind::PrefixOp {
                        symbol: symbol.clone(),
                        operand: Box::new(operand),
                    },
                    token.range,
                ))
            }

            ExprTokenKind::Bracketed(_) => self.parse_array_literal(token),

            ExprTokenKind::IntLiteral(value) => Some(UnresolvedExpr::new(
                UnresolvedExprKind::IntLiteral(*value),
                token.range,
            )),
            ExprTokenKind::FloatLiteral(value) => Some(UnresolvedExpr::new(
                UnresolvedExprKind::FloatLiteral(*value),
                token.range,
            )),
            ExprTokenKind::BoolLiteral(value) => Some(UnresolvedExpr::new(
                UnresolvedExprKind::BoolLiteral(*value),
                token.range,
            )),

            ExprTokenKind::Identifier(name) => Some(UnresolvedExpr::new(
                UnresolvedExprKind::Chain {
                    lhs: None,
                    elements: vec![UnresolvedChainElement::Identifier {
                        name: name.clone(),
                        range: token.range,
                    }],
                },
                token.range,
            )),

            ExprTokenKind::FuncCall { name, args } => Some(UnresolvedExpr::new(
                UnresolvedExprKind::Chain {
                    lhs: None,
                    elements: vec![UnresolvedChainElement::FuncCall {
                        name: name.clone(),
                        args: self.parse_func_args(args)?,
                        range: token.range,
                    }],
                },
                token.range,
            )),

            ExprTokenKind::UnresolvedExpr(expr) => Some(expr.clone()),

            ExprTokenKind::Parenthesized(_) => {
                self.ec.comp_bug(
                    token.range,
                    Ph::GlobalDeclCollection,
                    "Parenthesized expression should have already been parsed by build() function.",
                );
                None
            }

            ExprTokenKind::Dot => {
                self.ec
                    .expr_begins_with_invalid(token.range, Ph::ExprEngine, ".");
                None
            }

            ExprTokenKind::Semicolon => {
                self.ec
                    .expr_begins_with_invalid(token.range, Ph::ExprEngine, ";");
                None
            }

            ExprTokenKind::Comma => {
                self.ec
                    .expr_begins_with_invalid(token.range, Ph::ExprEngine, ",");
                None
            }
        }
    }

    pub fn parse_func_args(
        &mut self,
        args: &[ParserFuncCallArg],
    ) -> Option<Vec<NoTypeFuncCallArg>> {
        let mut parsed_args = Vec::new();
        for arg in args {
            let arg_expr = self.build(&arg.value)?;
            parsed_args.push(NoTypeFuncCallArg {
                label: arg.label.clone(),
                value: arg_expr,
                range: arg.range,
            });
        }
        Some(parsed_args)
    }
}
