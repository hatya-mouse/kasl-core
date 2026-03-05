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

use crate::{Expression, data::SymbolID};

#[derive(Debug, PartialEq, Clone)]
pub struct InputVar {
    pub name: String,
    pub value_type: SymbolID,
    pub def_val: Expression,
    pub attrs: Vec<InputAttribute>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputAttribute {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutputVar {
    pub name: String,
    pub value_type: SymbolID,
    pub def_val: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StateVar {
    pub name: String,
    pub value_type: SymbolID,
    pub def_val: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScopeVar {
    pub name: String,
    pub value_type: SymbolID,
    pub def_val: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncParam {
    pub label: Option<String>,
    pub name: String,
    pub value_type: SymbolID,
    pub def_val: Option<Expression>,
}
