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
    ast::{Expr, ExprKind, namespace_registry::ArrayID},
    lowerer::func_translator::FuncTranslator,
};
use kasl_ir::ir::{InstBuilder, Value};

impl FuncTranslator<'_> {
    pub(in crate::lowerer::func_translator::expr_translator) fn store_array(
        &mut self,
        expr: &Expr,
        array_id: &ArrayID,
        dst_ptr: Value,
        dst_offset: u32,
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
                for (i, item) in (0u32..).zip(items.iter()) {
                    let offset = dst_offset + item_size * i;
                    self.store_init_value(item, dst_ptr, offset);
                }
            }
            ExprKind::ArraySpread { value, count } => {
                if self.is_zero(value) {
                    // Use memset to fill the target memory with zeros
                    // Get the address of the slot
                    self.builder.memset(item_size * count, 0, dst_ptr);
                } else {
                    for i in 0..*count {
                        let offset = dst_offset + item_size * i;
                        self.store_init_value(value, dst_ptr, offset);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
