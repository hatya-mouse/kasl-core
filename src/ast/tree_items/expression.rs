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
    FuncCallArg, PrimitiveType, Program, Range,
    data::SymbolID,
    error::{ErrorCollector, Phase},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IntLiteral(u32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    PrefixOperator {
        operand: Box<Expression>,
        operand_type: SymbolID,
        return_type: SymbolID,
    },
    InfixOperator {
        lhs: Box<Expression>,
        lhs_type: SymbolID,
        rhs: Box<Expression>,
        rhs_type: SymbolID,
        return_type: SymbolID,
    },
    Identifier(SymbolID),
    FuncCall {
        id: SymbolID,
        args: Vec<FuncCallArg>,
    },
}

impl Expression {
    pub fn get_type(
        &self,
        ec: &mut ErrorCollector,
        program: &Program,
        error_range: Range,
    ) -> Option<SymbolID> {
        match self {
            Expression::IntLiteral(_) => program.get_id_of_primitive_type(&PrimitiveType::Int),
            Expression::FloatLiteral(_) => program.get_id_of_primitive_type(&PrimitiveType::Float),
            Expression::BoolLiteral(_) => program.get_id_of_primitive_type(&PrimitiveType::Bool),
            Expression::PrefixOperator { return_type, .. } => Some(*return_type),
            Expression::InfixOperator { return_type, .. } => Some(*return_type),
            Expression::Identifier(symbol_id) => match program.get_symbol_type(symbol_id) {
                Some(type_path) => Some(type_path),
                None => {
                    ec.comp_bug(
                        error_range,
                        Phase::TypeResolution,
                        &format!(
                            "Identifier with ID {} which should have been resolved",
                            symbol_id
                        ),
                    );
                    None
                }
            },
            Expression::FuncCall { id: func_id, .. } => match program.get_func(func_id) {
                Some(func) => func.return_type,
                None => {
                    ec.comp_bug(
                        error_range,
                        Phase::TypeResolution,
                        &format!(
                            "Call to function with ID {} which should have been resolved",
                            func_id
                        ),
                    );
                    None
                }
            },
        }
    }
}
