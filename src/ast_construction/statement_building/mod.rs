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

mod block_stmt_building;
mod body_collector;

pub use block_stmt_building::BlockStmtBuilder;

use crate::{
    ast::{CompilationData, compilation_data::ProgramContext},
    builtin::BuiltinRegistry,
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
