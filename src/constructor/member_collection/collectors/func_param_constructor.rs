//
// Copyright 2025-2026 Shuntaro Kasatani
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

use crate::{FuncParam, ParserFuncParam};

pub fn construct_func_params(parser_params: &[ParserFuncParam]) -> Vec<FuncParam> {
    parser_params
        .iter()
        .map(|param| FuncParam {
            label: param.label.clone(),
            name: param.name.clone(),
            value_type: None,
            def_val: None,
        })
        .collect()
}
