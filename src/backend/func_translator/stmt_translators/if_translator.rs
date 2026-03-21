use crate::{IfArm, backend::func_translator::FuncTranslator, symbol_table};
use cranelift::prelude::InstBuilder;
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_if(
        &mut self,
        main: &IfArm,
        else_ifs: &[IfArm],
        else_block: Option<&symbol_table::Block>,
        exit_block: ir::Block,
    ) {
        // Create a merge arm
        let merge_block = self.builder.create_block();
        let mut else_ir_block;

        // Translate the main arm
        else_ir_block = self.translate_if_arm(main, merge_block, exit_block);
        // Switch to the else block
        self.builder.switch_to_block(else_ir_block);
        self.builder.seal_block(else_ir_block);

        // Translate the if-else arms
        for arm in else_ifs {
            else_ir_block = self.translate_if_arm(arm, merge_block, exit_block);
            // Switch to the else block
            self.builder.switch_to_block(else_ir_block);
            self.builder.seal_block(else_ir_block);
        }

        // Build the else block
        if let Some(else_block) = else_block {
            let has_else_return = self.translate_block(else_block, exit_block);
            if !has_else_return {
                self.builder.ins().jump(merge_block, &[]);
            }
        } else {
            self.builder.ins().jump(merge_block, &[]);
        }

        // Switch to the merge block
        self.builder.switch_to_block(merge_block);
        self.builder.seal_block(merge_block);
    }

    /// Creates an if arm and returns the else block.
    pub fn translate_if_arm(
        &mut self,
        if_arm: &IfArm,
        merge_block: ir::Block,
        exit_block: ir::Block,
    ) -> ir::Block {
        // Create a else block
        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();

        // Translate the condition and conditionally branch.
        let cond = self.translate_expr(&if_arm.condition).unwrap();
        self.builder
            .ins()
            .brif(cond, then_block, &[], else_block, &[]);

        // Switch to the block and add instructions
        self.builder.switch_to_block(then_block);
        self.builder.seal_block(then_block);

        let has_return = self.translate_block(&if_arm.block, exit_block);

        // Jump to the merge block
        if !has_return {
            self.builder.ins().jump(merge_block, &[]);
        }

        // Return the else block
        else_block
    }
}
