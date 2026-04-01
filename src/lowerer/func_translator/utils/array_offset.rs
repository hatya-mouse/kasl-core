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

use kasl_ir::{Const, IRType, InstBuilder, IntBinOp, Value};

use crate::{
    ast::{Expr, type_registry::ResolvedType},
    lowerer::func_translator::FuncTranslator,
};

impl FuncTranslator<'_> {
    pub fn calculate_array_offset(
        &mut self,
        array_type: &ResolvedType,
        base_ptr: Value,
        index_expr: &Expr,
    ) -> Value {
        // Get the size of the item and the array count
        let ResolvedType::Array(array_id) = array_type else {
            unreachable!();
        };
        let array_decl = self
            .prog_ctx
            .type_registry
            .get_array_decl(array_id)
            .unwrap();
        let item_size = self
            .prog_ctx
            .type_registry
            .get_type_actual_size(array_decl.item_type())
            .unwrap();
        let array_count = array_decl.count();

        // Translate the index
        let translated_index = self.translate_expr(index_expr).unwrap();
        // Clamp the index by the max value and zero
        let max_index = self.builder.const_val(Const::I32(*array_count as i32 - 1));
        let zero = self.builder.const_val(Const::I32(0));
        let zero_clamped_index = self.builder.ibop(IntBinOp::Max, zero, translated_index);
        let clamped_index = self
            .builder
            .ibop(IntBinOp::Min, zero_clamped_index, max_index);

        // Calculate the offset
        let item_size_ir = self.builder.const_val(Const::I32(item_size as i32));
        let offset = self
            .builder
            .ibop(IntBinOp::Mul, clamped_index, item_size_ir);
        // Extend the offset value to the pointer type
        let ptr_type_offset = self.builder.iresize(offset, IRType::Ptr);
        // Calculate the pointer to the corresponding value by adding the offset to the base pointer
        self.builder.ibop(IntBinOp::Add, base_ptr, ptr_type_offset)
    }
}
