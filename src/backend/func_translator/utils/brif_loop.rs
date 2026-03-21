use cranelift::prelude::{InstBuilder, IntCC, types};
use cranelift_codegen::ir;

use crate::backend::func_translator::FuncTranslator;

impl FuncTranslator<'_> {
    pub fn create_loop<F>(&mut self, count: ir::Value, body_translator: F)
    where
        F: FnOnce(&mut FuncTranslator, ir::Value, ir::Block),
    {
        // Create blocks for loop
        let body_block = self.builder.create_block();
        let increment_block = self.builder.create_block();
        let end_block = self.builder.create_block();
        let loop_header_block = self.builder.create_block();

        // Create a variable to store the current loop index
        let i = self.builder.declare_var(types::I32);
        let zero_val = self.builder.ins().iconst(types::I32, 0);
        self.builder.def_var(i, zero_val);

        // Jump to the body or the return block depending on the index
        self.builder.ins().jump(loop_header_block, &[]);
        self.builder.switch_to_block(loop_header_block);

        let current_i = self.builder.use_var(i);
        let continue_loop = self
            .builder
            .ins()
            .icmp(IntCC::UnsignedLessThan, current_i, count);
        self.builder
            .ins()
            .brif(continue_loop, body_block, &[], end_block, &[]);

        // Create a body block and the return block
        self.builder.switch_to_block(body_block);
        self.builder.seal_block(body_block);

        body_translator(self, current_i, increment_block);

        // Add jump instruction to the increment block at the end of the body
        self.builder.ins().jump(increment_block, &[]);
        self.builder.switch_to_block(increment_block);
        self.builder.seal_block(increment_block);

        // Increment the index
        let one_val = self.builder.ins().iconst(types::I32, 1);
        let next_i = self.builder.ins().iadd(current_i, one_val);
        self.builder.def_var(i, next_i);

        // Add jump instruction to the loop header block
        self.builder.ins().jump(loop_header_block, &[]);
        self.builder.seal_block(loop_header_block);

        // Add return instruction to the return block
        self.builder.switch_to_block(end_block);
        self.builder.seal_block(end_block);
    }
}
