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

use crate::{LOOP_UNROLL_THRESHOLD, ast::symbol_table, backend::func_translator::FuncTranslator};
use cranelift::prelude::{InstBuilder, types};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_loop(
        &mut self,
        count: u32,
        block: &symbol_table::Block,
        exit_block: ir::Block,
    ) {
        // Translate the count into an IR value
        let ir_count = self.builder.ins().iconst(types::I32, count as i64);
        // Create a loop
        if count <= LOOP_UNROLL_THRESHOLD {
            for _ in 0..count {
                self.translate_block(block, exit_block);
            }
        } else {
            self.create_loop(ir_count, |_self, _, _| {
                _self.translate_block(block, exit_block);
            });
        }
    }
}
