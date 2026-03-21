mod block_stmt_building;
mod body_collector;

pub use block_stmt_building::BlockStmtBuilder;

use crate::{
    CompilationData, builtin::BuiltinRegistry, compilation_data::ProgramContext,
    error::ErrorCollector,
};

pub struct StatementBuilder<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: &'a mut CompilationData,
    builtin_registry: &'a BuiltinRegistry,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_data: &'a mut CompilationData,
        builtin_registry: &'a BuiltinRegistry,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            comp_data,
            builtin_registry,
        }
    }

    pub fn build_all(&mut self) {
        // Get all the IDs
        let func_ids = self.prog_ctx.func_ctx.get_all_func_ids();
        let infix_ids = self.prog_ctx.op_ctx.all_infix_ids();
        let prefix_ids = self.prog_ctx.op_ctx.all_prefix_ids();
        let postfix_ids = self.prog_ctx.op_ctx.all_postfix_ids();

        // Loop over the ids and build the function body
        for func_id in func_ids {
            self.build_func_body(func_id);
        }

        for op_id in infix_ids {
            self.build_infix_body(op_id);
        }

        for op_id in prefix_ids {
            self.build_prefix_body(op_id);
        }

        for op_id in postfix_ids {
            self.build_postfix_body(op_id);
        }
    }
}
