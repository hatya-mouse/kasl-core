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
    CompilationData, NameSpace, builtin::BuiltinRegistry, error::ErrorCollector,
    scope_manager::ScopeGraph,
};

pub struct StatementBuilder<'a> {
    ec: &'a mut ErrorCollector,
    namespace: &'a mut NameSpace,
    comp_data: &'a CompilationData,
    builtin_registry: &'a BuiltinRegistry,

    scope_graph: &'a mut ScopeGraph,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        namespace: &'a mut NameSpace,
        comp_data: &'a CompilationData,
        builtin_registry: &'a BuiltinRegistry,
        scope_graph: &'a mut ScopeGraph,
    ) -> Self {
        Self {
            ec,
            namespace,
            comp_data,
            builtin_registry,
            scope_graph,
        }
    }

    pub fn build_all(&mut self) {
        // Get all the IDs
        let func_ids = self.namespace.func_ctx.func_ids();
        let infix_ids = self.namespace.op_ctx.all_infix_ids();
        let prefix_ids = self.namespace.op_ctx.all_prefix_ids();
        let postfix_ids = self.namespace.op_ctx.all_postfix_ids();

        // Create a block statement builder
        let mut func_stmt_builder = BlockStmtBuilder::new(
            self.ec,
            self.namespace,
            self.comp_data,
            self.builtin_registry,
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
