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

use crate::{ConstructorError, ExprToken, Program, SymbolPath, SymbolTable, symbol_path};

pub enum TypedToken {
    Value(SymbolPath), // The type of the value
    Operator(String),
    LParen,
    RParen,
}

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
        let mut expr_iter = expr.iter().peekable();

        // 1. Convert tokens to TypedToken so we can easily look up their types
        // 2. Rearrange tokens to get reverse polish notation
        // 3. Evaluate the reverse polish notation to get the type of the expression

        Ok(symbol_path![])
    }
}
