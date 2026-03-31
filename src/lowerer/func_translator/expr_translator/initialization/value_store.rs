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

use kasl_ir::ir::{InstBuilder, Offset, Value};

use crate::{
    ast::{Expr, type_registry::ResolvedType},
    lowerer::func_translator::FuncTranslator,
};

impl FuncTranslator<'_> {
    pub(super) fn store_init_value(&mut self, expr: &Expr, dst_ptr: Value, dst_offset: u32) {
        match expr.value_type {
            ResolvedType::Primitive(_) => {
                let val = self.translate_expr(expr).unwrap();
                self.builder
                    .store(val, dst_ptr, Offset::Immediate(dst_offset));
            }
            ResolvedType::Array(array_id) => {
                self.store_array(expr, &array_id, dst_ptr, dst_offset);
            }
            ResolvedType::Struct(struct_id) => {
                self.store_init_fields(&struct_id, dst_ptr, dst_offset);
            }
        }
    }
}
