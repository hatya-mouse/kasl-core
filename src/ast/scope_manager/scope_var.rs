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

#[derive(Debug, PartialEq, Clone)]
pub struct ScopeVar {
    pub name: String,
    pub def_val: Expr<ResolvedType>,
    pub range: Range,
    pub var_kind: VariableKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableKind {
    Input { attrs: Vec<InputAttribute> },
    Output,
    State,
    Local,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputAttribute {
    pub name: String,
    pub args: Vec<Expr<ResolvedType>>,
    pub range: Range,
}
