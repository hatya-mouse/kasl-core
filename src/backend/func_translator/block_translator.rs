use cranelift_codegen::ir;

use crate::{Statement, backend::func_translator::FuncTranslator, symbol_table::Block};

impl FuncTranslator<'_> {
    /// Translates the given block. This method does not create any new blocks.
    pub fn translate_block(&mut self, block: &Block, exit_block: ir::Block) -> bool {
        // Loop over the statements in the function and translate them
        for stmt in &block.body {
            if self.translate_stmt(stmt, exit_block) {
                return true;
            }
        }
        false
    }

    fn translate_stmt(&mut self, stmt: &Statement, exit_block: ir::Block) -> bool {
        match stmt {
            Statement::Block { block } => {
                return self.translate_block(block, exit_block);
            }
            Statement::LocalVar { var_id } => self.translate_local_var(var_id),
            Statement::LocalConst { var_id } => self.translate_local_const(var_id),
            Statement::Assign { target, value } => self.translate_assign(target, value),
            Statement::Expression { expr } => {
                self.translate_expr(expr);
            }
            Statement::If {
                main,
                else_ifs,
                else_block,
            } => self.translate_if(main, else_ifs, else_block.as_ref(), exit_block),
            Statement::Return { value } => {
                self.translate_return(value, exit_block);
                return true;
            }
            Statement::Loop { count, body } => {
                self.translate_loop(*count, body, exit_block);
            }
        }
        false
    }
}
