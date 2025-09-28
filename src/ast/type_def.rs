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

use crate::{FuncParam, Function, Operator, Statement, Variable};

pub type SymbolPath = Vec<SymbolPathComponent>;

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolPathComponent {
    Field(String),
    Method(String),
    TypeDef(String),
    CompInt,
    CompFloat,
    CompBool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDef {
    pub name: String,
    pub inherits: Vec<SymbolPath>,
    pub vars: Vec<Variable>,
    pub inits: Vec<Initializer>,
    pub funcs: Vec<Function>,
    pub types: Vec<TypeDef>,
    pub operators: Vec<Operator>,
}

impl TypeDef {
    pub fn new(name: String) -> Self {
        TypeDef {
            name,
            inherits: Vec::new(),
            vars: Vec::new(),
            inits: Vec::new(),
            funcs: Vec::new(),
            types: Vec::new(),
            operators: Vec::new(),
        }
    }

    pub fn find_type_def(&self, name: &str) -> Option<&TypeDef> {
        self.types.iter().find(|s| s.name == name)
    }

    pub fn fine_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef> {
        self.types.iter_mut().find(|s| s.name == name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Initializer {
    pub literal_bind: Option<LiteralBind>,
    pub params: Vec<FuncParam>,
    pub body: Vec<Statement>,
    pub required_by: Option<SymbolPath>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralBind {
    IntLiteral,
    FloatLiteral,
    BoolLiteral,
}
