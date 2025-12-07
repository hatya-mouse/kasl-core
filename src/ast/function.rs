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

use crate::{Expression, Statement, TypeDef};

#[derive(Debug, PartialEq, Clone)]
pub struct Function<'a> {
    pub name: String,
    pub params: Vec<FuncParam<'a>>,
    pub return_type: Option<&'a TypeDef<'a>>,
    pub body: Vec<Statement<'a>>,
    pub required_by: Option<&'a TypeDef<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncParam<'a> {
    pub label: Option<String>,
    pub name: String,
    pub value_type: Option<&'a TypeDef<'a>>,
    pub def_val: Option<Box<Expression<'a>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncCallArg<'a> {
    pub label: String,
    pub value: Expression<'a>,
}
