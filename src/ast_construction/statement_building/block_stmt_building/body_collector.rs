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

use crate::{FunctionID, OperatorID, statement_building::BlockStmtBuilder};

impl BlockStmtBuilder<'_> {
    pub fn build_func_body(&mut self, func_id: FunctionID) {
        // Get the ScopeID of the block
        let mut resolved_stmts = Vec::new();
        if let Some(body) = self.func_body_map.get_body(&func_id) {
            for stmt in body {
                let Some(func) = self.comp_state.func_ctx.get_func(&func_id) else {
                    continue;
                };

                let Some(resolved_stmt) =
                    self.build_stmt(stmt, func.block.scope_id, func.return_type)
                else {
                    continue;
                };
                resolved_stmts.push(resolved_stmt);
            }
        }

        // Set the statement to the block
        if let Some(func) = self.comp_state.func_ctx.get_func_mut(&func_id) {
            func.block.set_stmt(resolved_stmts);
        }
    }

    pub fn build_infix_body(&mut self, op_id: OperatorID) {
        // Get the ScopeID of the block
        let mut resolved_stmts = Vec::new();
        if let Some(body) = self.op_body_map.get_body(&op_id) {
            for stmt in body {
                let Some(op) = self.comp_state.op_ctx.get_infix_op(&op_id) else {
                    continue;
                };

                let Some(resolved_stmt) =
                    self.build_stmt(stmt, op.block.scope_id, Some(op.return_type))
                else {
                    continue;
                };
                resolved_stmts.push(resolved_stmt);
            }
        }

        // Set the statement to the block
        if let Some(op) = self.comp_state.op_ctx.get_infix_op_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
    }

    pub fn build_prefix_body(&mut self, op_id: OperatorID) {
        // Get the ScopeID of the block
        let mut resolved_stmts = Vec::new();
        if let Some(body) = self.op_body_map.get_body(&op_id) {
            for stmt in body {
                let Some(op) = self.comp_state.op_ctx.get_prefix_op(&op_id) else {
                    continue;
                };

                let Some(resolved_stmt) =
                    self.build_stmt(stmt, op.block.scope_id, Some(op.return_type))
                else {
                    continue;
                };
                resolved_stmts.push(resolved_stmt);
            }
        }

        // Set the statement to the block
        if let Some(op) = self.comp_state.op_ctx.get_prefix_op_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
    }

    pub fn build_postfix_body(&mut self, op_id: OperatorID) {
        // Get the ScopeID of the block
        let mut resolved_stmts = Vec::new();
        if let Some(body) = self.op_body_map.get_body(&op_id) {
            for stmt in body {
                let Some(op) = self.comp_state.op_ctx.get_postfix_op(&op_id) else {
                    continue;
                };

                let Some(resolved_stmt) =
                    self.build_stmt(stmt, op.block.scope_id, Some(op.return_type))
                else {
                    continue;
                };
                resolved_stmts.push(resolved_stmt);
            }
        }

        // Set the statement to the block
        if let Some(op) = self.comp_state.op_ctx.get_postfix_op_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
    }
}
