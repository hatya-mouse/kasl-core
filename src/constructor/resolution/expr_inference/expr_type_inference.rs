//
// Copyright 2025-2026 Shuntaro Kasatani
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
    ConstructorError, ExprToken, Program, SymbolPath, SymbolTable, get_typed_tokens,
    resolution::expr_inference::{build_expr_tree, rearrange_tokens_to_rpn},
    symbol_path,
};

pub trait ExprTypeInference<'a> {
    fn infer_expr_type(
        &self,
        expr: &[ExprToken],
        symbol_table: &SymbolTable,
    ) -> Result<SymbolPath, ConstructorError>;
}

impl<'a> ExprTypeInference<'a> for Program {
    fn infer_expr_type(
        &self,
        expr: &[ExprToken],
        symbol_table: &SymbolTable,
    ) -> Result<SymbolPath, ConstructorError> {
        // 1. Convert tokens to TypedToken so we can easily look up their types
        let typed_tokens = get_typed_tokens(self, expr, symbol_table)?;
        // 2. Rearrange tokens to get reverse polish notation
        let rpn_tokens = rearrange_tokens_to_rpn(self, typed_tokens);
        // 3. Evaluate the reverse polish notation to get the type of the expression
        let expr_tree = build_expr_tree(self, rpn_tokens);

        Ok(symbol_path![])
    }
}
