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

use crate::ParserFuncCallArg;
use crate::expr_engine::ExpressionBuilder;
use crate::symbol_table::NoTypeFuncCallArg;
use crate::{
    Expr, ExprKind, ExprToken, ExprTokenKind, ParserMemberAccess, error::Ph,
    symbol_table::MemberAccess,
};
use std::{iter::Peekable, slice::Iter};

impl ExpressionBuilder<'_> {
    pub fn parse_lhs(&mut self, tokens: &mut Peekable<Iter<ExprToken>>) -> Option<Expr<()>> {
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
                let prefix_prec = match self.op_ctx.get_prefix_props(symbol) {
                    Some(op_props) => op_props.precedence,
                    None => {
                        self.ec
                            .prefix_op_not_found(token.range, Ph::ExprEngine, symbol);
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
                    no_type_args: self.parse_func_args(args)?,
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
                        no_type_args: self.parse_func_args(args)?,
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

    fn parse_func_args(&mut self, args: &[ParserFuncCallArg]) -> Option<Vec<NoTypeFuncCallArg>> {
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
