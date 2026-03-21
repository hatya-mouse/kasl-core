mod array_literal_parser;
mod climb_precedence;
mod lhs_chain_parser;
mod lhs_parser;

use crate::{
    ExprToken, ExprTokenKind,
    error::ErrorCollector,
    symbol_table::{OperatorContext, UnresolvedExpr},
};

pub struct ExpressionBuilder<'a> {
    ec: &'a mut ErrorCollector,
    op_ctx: &'a OperatorContext,
}

impl<'a> ExpressionBuilder<'a> {
    pub fn new(ec: &'a mut ErrorCollector, op_ctx: &'a OperatorContext) -> Self {
        Self { ec, op_ctx }
    }

    pub fn build(&mut self, tokens: &[ExprToken]) -> Option<UnresolvedExpr> {
        // First, build the parenthesized tokens by calling `build` recursively
        let mut processed_tokens: Vec<ExprToken> = Vec::new();
        for token in tokens {
            match &token.kind {
                ExprTokenKind::Parenthesized(inner) => {
                    processed_tokens.push(ExprToken {
                        kind: ExprTokenKind::UnresolvedExpr(self.build(inner)?),
                        range: token.range,
                    });
                }
                _ => {
                    processed_tokens.push(token.clone());
                }
            }
        }

        // Then, convert the processed tokens into an `Expression`
        let mut token_iter = processed_tokens.iter().peekable();
        self.climb_precedence(&mut token_iter, 0)
    }
}
