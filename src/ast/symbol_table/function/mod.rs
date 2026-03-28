//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

mod func_body_map;
mod function_def;

pub use func_body_map::FuncBodyMap;
pub use function_def::{FuncCallArg, FuncParam, Function, FunctionType, NoTypeFuncCallArg};

use crate::ast::{FunctionID, NameSpaceID, StructID};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct FunctionContext {
    funcs: HashMap<FunctionID, Function>,
    member_functions: HashMap<StructID, HashMap<String, FunctionID>>,
    global_functions: HashMap<(NameSpaceID, String), FunctionID>,
    next_function_id: usize,
}

impl FunctionContext {
    pub fn generate_function_id(&mut self) -> FunctionID {
        let id = FunctionID(self.next_function_id);
        self.next_function_id += 1;
        id
    }

    // --- GETTER FUNCTIONS ---

    pub fn get_global_func_id(&self, namespace_id: NameSpaceID, name: &str) -> Option<FunctionID> {
        self.global_functions
            .get(&(namespace_id, name.to_string()))
            .copied()
    }

    pub fn get_member_func_id(&self, struct_id: &StructID, name: &str) -> Option<FunctionID> {
        self.member_functions
            .get(struct_id)
            .and_then(|funcs| funcs.get(name))
            .copied()
    }

    pub fn get_func(&self, func_id: &FunctionID) -> Option<&Function> {
        self.funcs.get(func_id)
    }

    pub fn get_func_mut(&mut self, func_id: &FunctionID) -> Option<&mut Function> {
        self.funcs.get_mut(func_id)
    }

    pub fn get_all_func_ids(&self) -> Vec<FunctionID> {
        self.funcs.keys().copied().collect()
    }

    // --- REGISTRATION ---

    /// Registers a member function in the given struct in the namespace.
    pub fn register_member_func(&mut self, func: Function, struct_id: StructID) -> FunctionID {
        let func_id = self.generate_function_id();
        // Insert the function to the member functions map
        self.member_functions
            .entry(struct_id)
            .or_default()
            .insert(func.name.clone(), func_id);
        self.funcs.insert(func_id, func);
        func_id
    }

    /// Registers a global function in the given namespace.
    pub fn register_global_func(
        &mut self,
        namespace_id: NameSpaceID,
        func: Function,
    ) -> FunctionID {
        let func_id = self.generate_function_id();
        // Insert the function to the global functions map
        self.global_functions
            .insert((namespace_id, func.name.clone()), func_id);
        self.funcs.insert(func_id, func);
        func_id
    }
}
