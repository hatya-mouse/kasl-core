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

use std::collections::HashMap;

use crate::{ParserStatementKind, SymbolPath};

pub struct SymbolTable<'a> {
    pub symbol_def_table: HashMap<SymbolPath, &'a ParserStatementKind>,
    pub func_def_table: HashMap<SymbolPath, &'a ParserStatementKind>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            symbol_def_table: HashMap::new(),
            func_def_table: HashMap::new(),
        }
    }
}
