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

use crate::{Expr, Range, symbol_table::Block, type_registry::ResolvedType};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub is_member: bool,
    pub is_static: bool,
    pub params: Vec<FuncParam>,
    pub return_type: Option<ResolvedType>,
    pub block: Block,
    pub range: Range,
}

impl Function {
    pub fn get_param_name_by_label(&self, label: &str) -> Option<String> {
        self.params
            .iter()
            .find(|param| param.label.as_ref().is_some_and(|l| l == label) || param.name == label)
            .map(|param| param.name.to_string())
    }

    pub fn get_param_name_by_index(&self, index: usize) -> Option<String> {
        self.params.get(index).map(|param| param.name.to_string())
    }

    pub fn min_num_of_params(&self) -> usize {
        self.params
            .iter()
            .filter(|param| param.def_val.is_none())
            .count()
    }

    pub fn max_num_of_params(&self) -> usize {
        self.params.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoTypeFuncCallArg {
    pub label: Option<String>,
    pub value: Expr<()>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncCallArg {
    pub arg_name: String,
    pub value: Expr<ResolvedType>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncParam {
    pub label: Option<String>,
    pub name: String,
    pub value_type: ResolvedType,
    pub def_val: Option<Expr<ResolvedType>>,
    pub range: Range,
}
