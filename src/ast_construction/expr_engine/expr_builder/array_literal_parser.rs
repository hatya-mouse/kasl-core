use crate::{
    ExprToken, ExprTokenKind,
    expr_engine::ExpressionBuilder,
    symbol_table::{UnresolvedExpr, UnresolvedExprKind},
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
