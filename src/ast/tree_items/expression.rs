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
    FuncCallArg, LiteralBind, Program, Range, SymbolPath,
    error::{ErrorCollector, Phase},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IntLiteral(u32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    PrefixOperator {
        operand: Box<Expression>,
        operand_type: SymbolPath,
        return_type: SymbolPath,
    },
    InfixOperator {
        lhs: Box<Expression>,
        lhs_type: SymbolPath,
        rhs: Box<Expression>,
        rhs_type: SymbolPath,
        return_type: SymbolPath,
    },
    Identifier(SymbolPath),
    FuncCall {
        path: SymbolPath,
        args: Vec<FuncCallArg>,
    },
}

impl Expression {
    pub fn get_type(
        &self,
        ec: &mut ErrorCollector,
        program: &Program,
        error_range: Range,
    ) -> Option<SymbolPath> {
        match self {
            Expression::IntLiteral(_) => match &program.int_literal_type {
                Some(type_path) => Some(type_path.clone()),
                None => {
                    ec.no_literal_bind(error_range, Phase::TypeResolution, LiteralBind::IntLiteral);
                    None
                }
            },
            Expression::FloatLiteral(_) => match &program.float_literal_type {
                Some(type_path) => Some(type_path.clone()),
                None => {
                    ec.no_literal_bind(
                        error_range,
                        Phase::TypeResolution,
                        LiteralBind::FloatLiteral,
                    );
                    None
                }
            },
            Expression::BoolLiteral(_) => match program.bool_literal_type.clone() {
                Some(type_path) => Some(type_path),
                None => {
                    ec.no_literal_bind(
                        error_range,
                        Phase::TypeResolution,
                        LiteralBind::BoolLiteral,
                    );
                    None
                }
            },
            Expression::PrefixOperator { return_type, .. } => Some(return_type.clone()),
            Expression::InfixOperator { return_type, .. } => Some(return_type.clone()),
            Expression::Identifier(symbol_path) => Some(symbol_path.clone()),
            Expression::FuncCall { path, .. } => match program.get_func_by_path(path) {
                Some(func) => func.return_type.clone(),
                None => {
                    ec.func_not_found(error_range, Phase::TypeResolution, &path.to_string());
                    None
                }
            },
        }
    }
}
