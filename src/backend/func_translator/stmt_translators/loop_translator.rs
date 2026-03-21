use crate::{backend::func_translator::FuncTranslator, symbol_table};
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
        self.create_loop(ir_count, |_self, _, _| {
            _self.translate_block(block, exit_block);
        });
    }
}
