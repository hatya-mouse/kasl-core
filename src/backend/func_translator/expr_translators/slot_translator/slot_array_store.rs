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
    ast::namespace_registry::ArrayID,
    ast::{Expr, ExprKind},
    backend::func_translator::FuncTranslator,
};
use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir::StackSlot;
use cranelift_module::Module;

impl FuncTranslator<'_> {
    pub fn store_array_to_slot(
        &mut self,
        expr: &Expr,
        array_id: &ArrayID,
        slot: StackSlot,
        base_offset: i32,
    ) {
        // Get the size of the items
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

        match &expr.kind {
            ExprKind::ArrayList(items) => {
                for (i, item) in (0i32..).zip(items.iter()) {
                    let offset = base_offset + item_size as i32 * i;
                    self.store_value_to_slot(item, slot, offset);
                }
            }
            ExprKind::ArraySpread { value, count } => {
                if self.is_zero(value) {
                    // Use memset to fill the target memory with zeros
                    // Get the address of the slot
                    let ptr = self.builder.ins().stack_addr(
                        self.type_converter.pointer_type(),
                        slot,
                        base_offset,
                    );
                    self.builder.emit_small_memset(
                        self.module.target_config(),
                        ptr,
                        0,
                        item_size as u64 * *count as u64,
                        1,
                        MemFlags::new(),
                    );
                } else {
                    for i in 0..*count as i32 {
                        let offset = base_offset + item_size as i32 * i;
                        self.store_value_to_slot(value, slot, offset);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
