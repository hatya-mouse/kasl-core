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

use crate::{Function, FunctionID, StructID, type_registry::ResolvedType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FunctionContext {
    funcs: HashMap<FunctionID, Function>,
    member_functions: HashMap<StructID, HashMap<String, FunctionID>>,
    global_functions: HashMap<String, FunctionID>,
}

impl FunctionContext {
    pub fn get_type(&self, symbol_id: &FunctionID) -> Option<ResolvedType> {
        self.funcs
            .get(symbol_id)
            .and_then(|func| func.return_type.clone())
    }

    pub fn register_func(&mut self, func: Function, id: FunctionID) {
        self.funcs.insert(id, func);
    }

    pub fn get_func(&self, symbol_id: &FunctionID) -> Option<&Function> {
        self.funcs.get(symbol_id)
    }

    pub fn get_global_func_by_name(&self, name: &str) -> Option<&FunctionID> {
        self.global_functions.get(name)
    }

    pub fn get_member_func_by_name(&self, struct_id: &StructID, name: &str) -> Option<&FunctionID> {
        self.member_functions
            .get(struct_id)
            .and_then(|funcs| funcs.get(name))
    }
}
