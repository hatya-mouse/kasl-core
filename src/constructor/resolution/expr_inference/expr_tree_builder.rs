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
    ExprToken, Expression, Program, SymbolTable,
    error::ErrorCollector,
    get_typed_tokens,
    resolution::expr_inference::{build_expr_tree_from_rpn, rearrange_tokens_to_rpn},
};

pub trait ExprTreeBuilder {
    /// Build a typed Expression from `expr` tokens using the provided `SymbolTable`.
    fn build_expr_tree_from_raw_tokens(
        &self,
        ec: &mut ErrorCollector,
        expr: &[ExprToken],
        symbol_table: &SymbolTable,
    ) -> Option<Expression>;
}

impl ExprTreeBuilder for Program {
    fn build_expr_tree_from_raw_tokens(
        &self,
        ec: &mut ErrorCollector,
        expr: &[ExprToken],
        symbol_table: &SymbolTable,
    ) -> Option<Expression> {
        // 1. Convert tokens to TypedToken so we can easily look up their types
        let typed_tokens = get_typed_tokens(ec, self, expr)?;
        // 2. Rearrange tokens to get reverse polish notation
        let rpn_tokens = rearrange_tokens_to_rpn(ec, self, typed_tokens)?;
        // 3. Evaluate the reverse polish notation to get the type of the expression

        build_expr_tree_from_rpn(ec, self, symbol_table, rpn_tokens)
    }
}
