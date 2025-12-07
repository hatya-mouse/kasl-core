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

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDef<'a> {
    pub name: String,
    pub inherits: Vec<&'a TypeDef<'a>>,
    pub vars: Vec<Variable<'a>>,
    pub inits: Vec<Initializer<'a>>,
    pub funcs: Vec<Function<'a>>,
    pub types: Vec<TypeDef<'a>>,
    pub operators: Vec<Operator<'a>>,
}

impl<'a> TypeDef<'a> {
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

    pub fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef<'a>> {
        self.types.iter_mut().find(|s| s.name == name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Initializer<'a> {
    pub literal_bind: Option<LiteralBind>,
    pub params: Vec<FuncParam<'a>>,
    pub body: Vec<Statement<'a>>,
    pub required_by: Option<&'a TypeDef<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralBind {
    IntLiteral,
    FloatLiteral,
    BoolLiteral,
}
