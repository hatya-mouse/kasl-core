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

use crate::{FunctionID, ParserScopeStmt};
use std::collections::HashMap;

#[derive(Default)]
pub struct FuncBodyMap {
    pub func_map: HashMap<FunctionID, Vec<ParserScopeStmt>>,
}

impl FuncBodyMap {
    pub fn register(&mut self, func_id: FunctionID, body: Vec<ParserScopeStmt>) {
        self.func_map.insert(func_id, body);
    }

    pub fn get_body(&self, func_id: &FunctionID) -> Option<&Vec<ParserScopeStmt>> {
        self.func_map.get(func_id)
    }
}
