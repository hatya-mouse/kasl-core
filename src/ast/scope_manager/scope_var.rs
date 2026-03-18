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

use crate::{Expr, Range, type_registry::ResolvedType};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct ScopeVar {
    pub name: String,
    pub value_type: ResolvedType,
    pub def_val: Option<Expr>,
    pub range: Range,
    pub var_kind: VariableKind,
}

impl ScopeVar {
    pub fn expect_def_val(&self) -> &Expr {
        self.def_val.as_ref().unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum VariableKind {
    Input { attrs: Vec<InputAttribute> },
    Output,
    State,
    GlobalConst,
    FuncCallArg,
    LocalVar,
    LocalConst,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct InputAttribute {
    pub name: String,
    pub args: Vec<Expr>,
    pub range: Range,
}
