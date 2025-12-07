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

use crate::{Expression, TypeDef};

#[derive(Debug, PartialEq, Clone)]
pub struct InputAttribute<'a> {
    pub name: String,
    pub args: Vec<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputVar<'a> {
    pub name: String,
    pub value_type: Option<&'a TypeDef<'a>>,
    pub def_val: Option<Expression<'a>>,
    pub attrs: Vec<InputAttribute<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutputVar<'a> {
    pub name: String,
    pub value_type: Option<&'a TypeDef<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateVar<'a> {
    pub name: String,
    pub value_type: Option<&'a TypeDef<'a>>,
    pub def_val: Option<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable<'a> {
    pub required_by: Option<&'a TypeDef<'a>>,
    pub name: String,
    pub value_type: Option<&'a TypeDef<'a>>,
    pub def_val: Option<Expression<'a>>,
}
