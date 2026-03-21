use crate::{Expr, backend::func_translator::FuncTranslator};
use cranelift::prelude::InstBuilder;
use cranelift_codegen::ir::{self, BlockArg};

impl FuncTranslator<'_> {
    pub fn translate_return(&mut self, value: &Option<Expr>, exit_block: ir::Block) {
        if let Some(return_val) = value.as_ref().and_then(|val| self.translate_expr(val)) {
            self.builder
                .ins()
                .jump(exit_block, &[BlockArg::Value(return_val)]);
        } else {
            self.builder.ins().jump(exit_block, &[]);
        }
    }
}
