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

use crate::{
    ast_nodes::{FunctionID, NameSpaceID, OperatorID, ScopeID, flow_graph::FlowGraphBuilder},
    semantic_analysis::stmt_builder::{BlockStmtBuilder, StatementBuilder},
};

impl StatementBuilder<'_> {
    pub fn build_func_body(&mut self, func_id: FunctionID) {
        // Get a reference to the function
        let Some(func) = self.prog_ctx.func_ctx.get_func(&func_id) else {
            return;
        };
        let Some(body) = self.comp_data.func_body_map.get_body(&func_id).cloned() else {
            return;
        };

        let func_namespace = func.namespace_id;
        let func_scope = func.block.scope_id;
        let func_return_type = func.return_type;

        // Create a block statement builder
        let mut flow_graph_builder = FlowGraphBuilder::with_entry_node();
        let mut block_builder = BlockStmtBuilder::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            &mut flow_graph_builder,
            func_scope,
            func_namespace,
            func_return_type,
        );
        // Build the statements in the function
        let resolved_stmts = block_builder.build_statements(&body);

        // Add the built flow graph to the compilation data
        let flow_graph = flow_graph_builder.take_graph();
        self.comp_data.func_flow_graphs.insert(func_id, flow_graph);

        if let Some(func) = self.prog_ctx.func_ctx.get_func_mut(&func_id) {
            // Set the statement to the block
            func.block.set_stmt(resolved_stmts);
        }
        // Add a function edge to the scope graph
        self.add_func_scope_edge(func_namespace, func_scope);
    }

    pub fn build_infix_body(&mut self, op_id: OperatorID) {
        // Get a reference to the operator
        let Some(op) = self.prog_ctx.op_ctx.get_infix_func(&op_id) else {
            return;
        };
        let Some(body) = self.comp_data.op_body_map.get_body(&op_id).cloned() else {
            return;
        };

        let op_namespace = op.namespace_id;
        let op_scope = op.block.scope_id;
        let op_return_type = op.return_type;

        // Create a block statement builder
        let mut flow_graph_builder = FlowGraphBuilder::with_entry_node();
        let mut block_builder = BlockStmtBuilder::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            &mut flow_graph_builder,
            op_scope,
            op_namespace,
            op_return_type,
        );
        // Build the statements in the operator
        let resolved_stmts = block_builder.build_statements(&body);

        // Add the built flow graph to the compilation data
        let flow_graph = flow_graph_builder.take_graph();
        self.comp_data.op_flow_graphs.insert(op_id, flow_graph);

        // Set the statement to the block
        if let Some(op) = self.prog_ctx.op_ctx.get_infix_func_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
        // Add an operator edge to the scope graph
        self.add_func_scope_edge(op_namespace, op_scope);
    }

    pub fn build_prefix_body(&mut self, op_id: OperatorID) {
        // Get a reference to the operator
        let Some(op) = self.prog_ctx.op_ctx.get_prefix_func(&op_id) else {
            return;
        };
        let Some(body) = self.comp_data.op_body_map.get_body(&op_id).cloned() else {
            return;
        };

        let op_namespace = op.namespace_id;
        let op_scope = op.block.scope_id;
        let op_return_type = op.return_type;

        // Create a block statement builder
        let mut flow_graph_builder = FlowGraphBuilder::with_entry_node();
        let mut block_builder = BlockStmtBuilder::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            &mut flow_graph_builder,
            op_scope,
            op_namespace,
            op_return_type,
        );
        // Build the statements in the operator
        let resolved_stmts = block_builder.build_statements(&body);

        // Add the built flow graph to the compilation data
        let flow_graph = flow_graph_builder.take_graph();
        self.comp_data.op_flow_graphs.insert(op_id, flow_graph);

        // Set the statement to the block
        if let Some(op) = self.prog_ctx.op_ctx.get_prefix_func_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
        // Add an operator edge to the scope graph
        self.add_func_scope_edge(op_namespace, op_scope);
    }

    pub fn build_postfix_body(&mut self, op_id: OperatorID) {
        // Get a reference to the operator
        let Some(op) = self.prog_ctx.op_ctx.get_postfix_func(&op_id) else {
            return;
        };
        let Some(body) = self.comp_data.op_body_map.get_body(&op_id).cloned() else {
            return;
        };

        let op_namespace = op.namespace_id;
        let op_scope = op.block.scope_id;
        let op_return_type = op.return_type;

        // Create a block statement builder
        let mut flow_graph_builder = FlowGraphBuilder::with_entry_node();
        let mut block_builder = BlockStmtBuilder::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            &mut flow_graph_builder,
            op_scope,
            op_namespace,
            op_return_type,
        );
        // Build the statements in the operator
        let resolved_stmts = block_builder.build_statements(&body);

        // Add the built flow graph to the compilation data
        let flow_graph = flow_graph_builder.take_graph();
        self.comp_data.op_flow_graphs.insert(op_id, flow_graph);

        // Set the statement to the block
        if let Some(op) = self.prog_ctx.op_ctx.get_postfix_func_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
        // Add an operator edge to the scope graph
        self.add_func_scope_edge(op_namespace, op_scope);
    }

    fn add_func_scope_edge(&mut self, func_namespace: NameSpaceID, func_scope: ScopeID) {
        // Register the function to the scope graph
        let global_scope_id = self
            .prog_ctx
            .scope_registry
            .get_global_scope_id(&func_namespace);
        // Add an edge from the global scope to the function
        self.comp_data
            .scope_graph
            .add_edge(global_scope_id, func_scope);
    }
}
