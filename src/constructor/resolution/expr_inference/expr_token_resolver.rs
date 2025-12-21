//
// Copyright 2025 Shuntaro Kasatani
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
    ConstructorError, ExprToken, ExprTokenKind, Program, SymbolPath, SymbolTable,
    resolution::expr_inference::{
        expr_type_inference::ResolvedToken, token_type_collector::collect_token_type,
    },
};

enum ExpectedTokenKind {
    ValueOrPrefix,
    InfixOrPostfix,
}

impl<'a> Program {
    fn resolve_expr_tokens(
        &self,
        expr: &[ExprToken],
        symbol_table: &SymbolTable,
    ) -> Result<Vec<ResolvedToken<'a>>, ConstructorError> {
        let token_types = collect_token_type(self, expr, symbol_table)?;
        let next_expected = ExpectedTokenKind::ValueOrPrefix;
        let mut result = Vec::new();

        for (expr, token_type) in expr.iter().zip(token_types.iter()) {
            if matches!(next_expected, ExpectedTokenKind::ValueOrPrefix) {
                match expr.kind {
                    ExprTokenKind::IntLiteral(_) => {
                        result.push(ResolvedToken::Value(SymbolPath::comp_int()));
                    }
                    ExprTokenKind::FloatLiteral(_) => {
                        result.push(ResolvedToken::Value(SymbolPath::comp_float()));
                    }
                    ExprTokenKind::BoolLiteral(_) => {
                        result.push(ResolvedToken::Value(SymbolPath::comp_bool()));
                    }
                    // TODO
                    _ => (),
                }
            } else {
            }
        }

        Ok(vec![])
    }
}
