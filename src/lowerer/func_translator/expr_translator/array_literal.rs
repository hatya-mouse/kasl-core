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

use crate::{
    ast::{Expr, type_registry::ResolvedType},
    lowerer::func_translator::FuncTranslator,
};
use kasl_ir::Value;

impl FuncTranslator<'_> {
    pub(super) fn translate_array_literal(&mut self, array_expr: &Expr) -> Value {
        // Assume the type is array
        let array_id = match array_expr.value_type {
            ResolvedType::Array(array_id) => array_id,
            _ => unreachable!(),
        };

        // Allocate memory for the array
        let ptr = self.alloc_array(&array_id);

        // Store the array items in the allocated memory
        self.store_array(array_expr, &array_id, ptr, 0);

        // Return the pointer to the array
        ptr
    }
}
