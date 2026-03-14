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

mod builtin_func;
mod functions;
mod utility;

pub use builtin_func::{BuiltinFunc, BuiltinFuncID};

use crate::type_registry::{PrimitiveType, ResolvedType};
use cranelift::prelude::FunctionBuilder;
use cranelift_codegen::ir;
use std::collections::HashMap;

pub struct BuiltinRegistry {
    functions: HashMap<BuiltinFuncID, BuiltinFunc>,
    next_builtin_func_id: usize,
}

impl Default for BuiltinRegistry {
    fn default() -> Self {
        let mut registry = Self::new();

        functions::int_op::register_builtins(&mut registry);
        functions::float_op::register_builtins(&mut registry);
        functions::logical::register_builtins(&mut registry);
        functions::type_conversion::register_builtins(&mut registry);

        registry
    }
}

impl BuiltinRegistry {
    fn new() -> Self {
        Self {
            functions: HashMap::new(),
            next_builtin_func_id: 0,
        }
    }

    fn generate_id(&mut self) -> BuiltinFuncID {
        let id = BuiltinFuncID::new(self.next_builtin_func_id);
        self.next_builtin_func_id += 1;
        id
    }

    pub(in crate::builtin) fn register_func(
        &mut self,
        name: &'static str,
        params: &[PrimitiveType],
        return_type: PrimitiveType,
        translator: Box<dyn Fn(&mut FunctionBuilder, &[ir::Value]) -> ir::Value>,
    ) {
        let func_id = self.generate_id();
        let func = BuiltinFunc {
            name,
            params: params
                .iter()
                .map(|ty| ResolvedType::Primitive(*ty))
                .collect(),
            return_type: ResolvedType::Primitive(return_type),
            translator,
        };
        self.functions.insert(func_id, func);
    }
}
