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

use std::fmt::Display;

use crate::{Expression, FuncParam, Statement, SymbolPath};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<FuncParam>,
    pub return_type: Option<SymbolPath>,
    pub required_by: Option<SymbolPath>,
    pub body: Vec<Statement>,
}

impl Function {
    pub fn get_func_param_mut(&mut self, name: &str) -> Option<&mut FuncParam> {
        self.params.iter_mut().find(|param| param.name == name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncCallArg {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Initializer {
    pub literal_bind: Option<LiteralBind>,
    pub params: Vec<FuncParam>,
    pub required_by: Option<SymbolPath>,
    pub body: Vec<Statement>,
}

impl Initializer {
    pub fn does_params_match(&self, param_types: &[SymbolPath]) -> bool {
        self.params
            .iter()
            .zip(param_types)
            .all(|(param, ty)| &param.value_type == ty)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LiteralBind {
    IntLiteral,
    FloatLiteral,
    BoolLiteral,
}

impl Display for LiteralBind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralBind::IntLiteral => write!(f, "intliteral"),
            LiteralBind::FloatLiteral => write!(f, "floatliteral"),
            LiteralBind::BoolLiteral => write!(f, "boolliteral"),
        }
    }
}
