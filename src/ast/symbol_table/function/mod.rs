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

mod func_body_map;
mod function_def;

pub use func_body_map::FuncBodyMap;
pub use function_def::{FuncCallArg, FuncParam, Function, NoTypeFuncCallArg};

use crate::{FunctionID, StructID, type_registry::ResolvedType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FunctionContext {
    funcs: HashMap<FunctionID, Function>,
    member_functions: HashMap<StructID, HashMap<String, FunctionID>>,
    static_functions: HashMap<StructID, HashMap<String, FunctionID>>,
    global_functions: HashMap<String, FunctionID>,
}

impl FunctionContext {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
            member_functions: HashMap::new(),
            static_functions: HashMap::new(),
            global_functions: HashMap::new(),
        }
    }

    pub fn get_type(&self, symbol_id: &FunctionID) -> Option<ResolvedType> {
        self.funcs
            .get(symbol_id)
            .and_then(|func| func.return_type.clone())
    }

    pub fn register_member_func(
        &mut self,
        func: Function,
        struct_id: StructID,
        func_id: FunctionID,
    ) {
        self.member_functions
            .entry(struct_id)
            .or_insert_with(HashMap::new)
            .insert(func.name.clone(), func_id);
        self.funcs.insert(func_id, func);
    }

    pub fn register_static_func(
        &mut self,
        func: Function,
        struct_id: StructID,
        func_id: FunctionID,
    ) {
        self.static_functions
            .entry(struct_id)
            .or_insert_with(HashMap::new)
            .insert(func.name.clone(), func_id);
        self.funcs.insert(func_id, func);
    }

    pub fn register_global_func(&mut self, func: Function, func_id: FunctionID) {
        self.global_functions.insert(func.name.clone(), func_id);
        self.funcs.insert(func_id, func);
    }

    pub fn get_global_func_by_name(&self, name: &str) -> Option<FunctionID> {
        self.global_functions.get(name).copied()
    }

    pub fn get_member_func_by_name(&self, struct_id: &StructID, name: &str) -> Option<FunctionID> {
        self.member_functions
            .get(struct_id)
            .and_then(|funcs| funcs.get(name))
            .copied()
    }

    pub fn get_func(&self, symbol_id: &FunctionID) -> Option<&Function> {
        self.funcs.get(symbol_id)
    }

    pub fn get_func_mut(&mut self, symbol_id: &FunctionID) -> Option<&mut Function> {
        self.funcs.get_mut(symbol_id)
    }

    pub fn func_ids(&self) -> Vec<FunctionID> {
        self.funcs.keys().copied().collect()
    }
}
