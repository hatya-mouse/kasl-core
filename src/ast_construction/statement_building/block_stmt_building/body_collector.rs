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

use crate::{FunctionID, OperatorID, ScopeID, statement_building::BlockStmtBuilder};

impl BlockStmtBuilder<'_> {
    pub fn build_func_body(&mut self, func_id: FunctionID) {
        // Get a reference to the function
        let Some(func) = self.namespace.func_ctx.get_func(&func_id) else {
            return;
        };
        let Some(body) = self.comp_data.func_body_map.get_body(&func_id) else {
            return;
        };

        // Build the statements in the function
        let resolved_stmts = self.build_statements(body, func.block.scope_id, func.return_type);

        if let Some(func) = self.namespace.func_ctx.get_func_mut(&func_id) {
            // Set the statement to the block
            func.block.set_stmt(resolved_stmts);
            // Add a function edge to the scope graph
            let func_scope = func.block.scope_id;
            let requires_return = !func.return_type.is_void();
            self.add_func_scope_edge(func_scope, requires_return);
        }
    }

    pub fn build_infix_body(&mut self, op_id: OperatorID) {
        // Get a reference to the operator
        let Some(op) = self.namespace.op_ctx.get_infix_op(&op_id) else {
            return;
        };
        let Some(body) = self.comp_data.op_body_map.get_body(&op_id) else {
            return;
        };

        // Build the statements in the operator
        let resolved_stmts = self.build_statements(body, op.block.scope_id, op.return_type);

        // Set the statement to the block
        if let Some(op) = self.namespace.op_ctx.get_infix_op_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
            // Add an operator edge to the scope graph
            let op_scope = op.block.scope_id;
            self.add_func_scope_edge(op_scope, true);
        }
    }

    pub fn build_prefix_body(&mut self, op_id: OperatorID) {
        // Get a reference to the operator
        let Some(op) = self.namespace.op_ctx.get_prefix_op(&op_id) else {
            return;
        };
        let Some(body) = self.comp_data.op_body_map.get_body(&op_id) else {
            return;
        };

        // Build the statements in the operator
        let resolved_stmts = self.build_statements(body, op.block.scope_id, op.return_type);

        // Set the statement to the block
        if let Some(op) = self.namespace.op_ctx.get_prefix_op_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
            // Add an operator edge to the scope graph
            let op_scope = op.block.scope_id;
            self.add_func_scope_edge(op_scope, true);
        }
    }

    pub fn build_postfix_body(&mut self, op_id: OperatorID) {
        // Get a reference to the operator
        let Some(op) = self.namespace.op_ctx.get_postfix_op(&op_id) else {
            return;
        };
        let Some(body) = self.comp_data.op_body_map.get_body(&op_id) else {
            return;
        };

        // Build the statements in the operator
        let resolved_stmts = self.build_statements(body, op.block.scope_id, op.return_type);

        // Set the statement to the block
        if let Some(op) = self.namespace.op_ctx.get_postfix_op_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
            // Add an operator edge to the scope graph
            let op_scope = op.block.scope_id;
            self.add_func_scope_edge(op_scope, true);
        }
    }

    fn add_func_scope_edge(&mut self, func_scope: ScopeID, requires_return: bool) {
        // Register the function to the scope graph
        let global_scope_id = self.namespace.scope_registry.get_global_scope_id();
        // Add an edge from the global scope to the function
        self.scope_graph.add_edge(global_scope_id, func_scope);
        // Mark the function scope as requires return
        self.scope_graph
            .set_requires_return(func_scope, requires_return);
    }
}
