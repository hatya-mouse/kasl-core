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

use crate::{Expression, FuncParam, Function, Operator, Variable};

#[derive(Debug, PartialEq, Clone)]
pub enum TypeName {
    CompInt,
    CompFloat,
    CompBool,
    Struct(String),
    Protocol(String),
}

impl TypeName {
    pub fn str(&self) -> &str {
        match self {
            TypeName::CompInt => "CompInt",
            TypeName::CompFloat => "CompFloat",
            TypeName::CompBool => "CompBool",
            TypeName::Struct(name) => name,
            TypeName::Protocol(name) => name,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructType {
    pub name: String,
    pub inherits: Vec<TypeName>,
    pub vars: Vec<Variable>,
    pub inits: Vec<Initializer>,
    pub funcs: Vec<Function>,
    pub ops: Vec<Operator>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProtocolType {
    name: String,
    inherits: Vec<TypeName>,
    funcs: Vec<Function>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Initializer {
    params: Vec<FuncParam>,
    body: Box<Expression>,
}
