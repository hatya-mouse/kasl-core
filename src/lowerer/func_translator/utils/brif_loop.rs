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

use crate::lowerer::func_translator::FuncTranslator;
use kasl_ir::{Block, Const, IRType, InstBuilder, IntBinOp, IntCmp, Value};

impl FuncTranslator<'_> {
    pub fn create_loop<F>(&mut self, count: Value, body_translator: F)
    where
        F: FnOnce(&mut FuncTranslator, Value, Block),
    {
        // Create blocks for loop management
        let body_block = self.builder.create_block(&[]);
        let increment_block = self.builder.create_block(&[]);
        let end_block = self.builder.create_block(&[]);
        let loop_header_block = self.builder.create_block(&[]);

        // Create a variable to store the current loop index
        let i = self.builder.create_var(IRType::I32);
        let zero_val = self.builder.const_val(Const::I32(0));
        self.builder.assign(i, zero_val);

        // Jump to th ebody or the return block depending on the index
        self.builder.jump(loop_header_block, &[]);
        self.builder.switch_to_block(loop_header_block);

        // Conditionally continue the loop or jump to the end block using brif
        let current_i = self.builder.load_var(i);
        let continue_loop = self.builder.icmp(IntCmp::Ult, current_i, count);
        self.builder
            .brif(continue_loop, body_block, &[], end_block, &[]);

        // Create a body block and the return block
        self.builder.switch_to_block(body_block);

        body_translator(self, current_i, increment_block);

        // Add jump instruction to the increment block at the end of the body block
        self.builder.jump(increment_block, &[]);
        self.builder.switch_to_block(increment_block);

        // Increment the index
        let one_val = self.builder.const_val(Const::I32(1));
        let next_i = self.builder.ibop(IntBinOp::Add, current_i, one_val);
        self.builder.assign(i, next_i);

        // Add jump instruction to the loop header block
        self.builder.jump(loop_header_block, &[]);

        // Switch to the end block to continue translating the rest of the function
        self.builder.switch_to_block(end_block);
    }
}
