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

use crate::{Expression, FuncParam, Statement, data::SymbolID};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub is_static: bool,
    pub params: Vec<FuncParam>,
    pub return_type: Option<SymbolID>,
    pub body: Vec<Statement>,
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
pub struct FuncCallArg {
    pub name: String,
    pub value: Expression,
}
