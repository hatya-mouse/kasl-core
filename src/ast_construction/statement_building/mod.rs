//
// © 2025-2026 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

mod block_stmt_building;

pub use block_stmt_building::BlockStmtBuilder;

use crate::{
    CompilationState, NameSpace,
    error::ErrorCollector,
    scope_manager::ScopeGraph,
    symbol_table::{FuncBodyMap, OpBodyMap},
};

pub struct StatementBuilder<'a> {
    ec: &'a mut ErrorCollector,
    name_space: &'a mut NameSpace,
    func_body_map: &'a FuncBodyMap,
    op_body_map: &'a OpBodyMap,
    comp_state: &'a mut CompilationState,

    scope_graph: &'a mut ScopeGraph,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        name_space: &'a mut NameSpace,
        func_body_map: &'a FuncBodyMap,
        op_body_map: &'a OpBodyMap,
        comp_state: &'a mut CompilationState,
        scope_graph: &'a mut ScopeGraph,
    ) -> Self {
        Self {
            ec,
            name_space,
            func_body_map,
            op_body_map,
            comp_state,
            scope_graph,
        }
    }

    pub fn build_all(&mut self) {
        // Get all the IDs
        let func_ids = self.comp_state.func_ctx.func_ids();
        let infix_ids = self.comp_state.op_ctx.all_infix_ids();
        let prefix_ids = self.comp_state.op_ctx.all_prefix_ids();
        let postfix_ids = self.comp_state.op_ctx.all_postfix_ids();

        // Create a block statement builder
        let mut func_stmt_builder = BlockStmtBuilder::new(
            self.ec,
            self.name_space,
            self.func_body_map,
            self.op_body_map,
            self.comp_state,
            self.scope_graph,
        );

        // Loop over the ids and build the function body
        for func_id in func_ids {
            func_stmt_builder.build_func_body(func_id);
        }

        for op_id in infix_ids {
            func_stmt_builder.build_infix_body(op_id);
        }

        for op_id in prefix_ids {
            func_stmt_builder.build_prefix_body(op_id);
        }

        for op_id in postfix_ids {
            func_stmt_builder.build_postfix_body(op_id);
        }
    }
}
