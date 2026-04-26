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

use crate::{
    ast_nodes::{Expr, ExprKind},
    lowerer::func_translator::FuncTranslator,
};

impl FuncTranslator<'_> {
    /// Checks if the passed expression is a zero literal.
    pub fn is_zero(&mut self, expr: &Expr) -> bool {
        match &expr.kind {
            ExprKind::BoolLiteral(value) => !value,
            ExprKind::FloatLiteral(value) => value == &0.0,
            ExprKind::IntLiteral(value) => value == &0,
            _ => false,
        }
    }
}
