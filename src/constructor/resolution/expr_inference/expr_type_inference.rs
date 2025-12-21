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

use crate::{ConstructorError, ExprToken, Operator, Program, SymbolPath, SymbolTable, symbol_path};

pub enum ResolvedToken<'a> {
    Value(SymbolPath), // The type of the value
    Operator(&'a Operator),
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

        Ok(symbol_path![])
    }
}
