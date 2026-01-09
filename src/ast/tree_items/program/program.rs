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

use crate::{Function, InputVar, OutputVar, StateVar, SymbolPath, TypeDef};

pub struct Program {
    pub main_func: Option<Function>,
    pub funcs: Vec<Function>,
    pub types: Vec<TypeDef>,
    pub states: Vec<StateVar>,
    pub inputs: Vec<InputVar>,
    pub outputs: Vec<OutputVar>,

    pub bool_literal_type: Option<SymbolPath>,
    pub int_literal_type: Option<SymbolPath>,
    pub float_literal_type: Option<SymbolPath>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            main_func: None,
            funcs: Vec::new(),
            types: Vec::new(),
            states: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            bool_literal_type: None,
            int_literal_type: None,
            float_literal_type: None,
        }
    }
}
