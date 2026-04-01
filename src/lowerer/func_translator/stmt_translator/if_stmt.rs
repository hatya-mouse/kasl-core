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
    ast::{IfArm, symbol_table},
    lowerer::func_translator::FuncTranslator,
};
use kasl_ir::{self, InstBuilder};

impl FuncTranslator<'_> {
    pub(super) fn translate_if(
        &mut self,
        main: &IfArm,
        else_ifs: &[IfArm],
        else_block: Option<&symbol_table::Block>,
        exit_block: kasl_ir::Block,
    ) {
        // Create a merge block
        let merge_block = self.builder.create_block(&[]);
        let mut else_ir_block;

        // Translate the main arm
        else_ir_block = self.translate_if_arm(main, merge_block, exit_block);
        // Switch to the else block
        self.builder.switch_to_block(else_ir_block);

        // Translate the if-else arms
        for arm in else_ifs {
            else_ir_block = self.translate_if_arm(arm, merge_block, exit_block);
            // Switch to the else block
            self.builder.switch_to_block(else_ir_block);
        }

        // Translate the contents of else block
        if let Some(else_block) = else_block {
            let has_else_return = self.translate_block(else_block, exit_block);
            if !has_else_return {
                self.builder.jump(merge_block, &[]);
            }
        }

        // Switch to the merge block and continue translating the rest of the function
        self.builder.switch_to_block(merge_block);
    }

    /// Translates and if arm and returns the else block for it.
    fn translate_if_arm(
        &mut self,
        if_arm: &IfArm,
        merge_block: kasl_ir::Block,
        exit_block: kasl_ir::Block,
    ) -> kasl_ir::Block {
        // Create a then block and an else block
        let then_block = self.builder.create_block(&[]);
        let else_block = self.builder.create_block(&[]);

        // Translate the condition and conditionally branch
        let cond = self.translate_expr(&if_arm.condition).unwrap();
        self.builder.brif(cond, then_block, &[], else_block, &[]);

        // Switch to the block and add instructions
        self.builder.switch_to_block(then_block);

        let has_return = self.translate_block(&if_arm.block, exit_block);

        // Jump to the merge block
        if !has_return {
            self.builder.jump(merge_block, &[]);
        }

        // Return the else block
        else_block
    }
}
