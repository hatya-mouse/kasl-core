use crate::{
    FunctionID, NameSpaceID, OperatorID, ScopeID,
    statement_building::{BlockStmtBuilder, StatementBuilder},
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
        let mut block_builder = BlockStmtBuilder::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            func_scope,
            func_namespace,
            func_return_type,
        );
        // Build the statements in the function
        let resolved_stmts = block_builder.build_statements(&body);

        if let Some(func) = self.prog_ctx.func_ctx.get_func_mut(&func_id) {
            // Set the statement to the block
            func.block.set_stmt(resolved_stmts);
        }
        // Add a function edge to the scope graph
        let requires_return = !func_return_type.is_void();
        self.add_func_scope_edge(func_namespace, func_scope, requires_return);
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
        let mut block_builder = BlockStmtBuilder::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            op_scope,
            op_namespace,
            op_return_type,
        );
        // Build the statements in the operator
        let resolved_stmts = block_builder.build_statements(&body);

        // Set the statement to the block
        if let Some(op) = self.prog_ctx.op_ctx.get_infix_func_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
        // Add an operator edge to the scope graph
        self.add_func_scope_edge(op_namespace, op_scope, true);
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
        let mut block_builder = BlockStmtBuilder::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            op_scope,
            op_namespace,
            op_return_type,
        );
        // Build the statements in the operator
        let resolved_stmts = block_builder.build_statements(&body);

        // Set the statement to the block
        if let Some(op) = self.prog_ctx.op_ctx.get_prefix_func_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
        // Add an operator edge to the scope graph
        self.add_func_scope_edge(op_namespace, op_scope, true);
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
        let mut block_builder = BlockStmtBuilder::new(
            self.ec,
            self.prog_ctx,
            self.comp_data,
            self.builtin_registry,
            op_scope,
            op_namespace,
            op_return_type,
        );
        // Build the statements in the operator
        let resolved_stmts = block_builder.build_statements(&body);

        // Set the statement to the block
        if let Some(op) = self.prog_ctx.op_ctx.get_postfix_func_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
        // Add an operator edge to the scope graph
        self.add_func_scope_edge(op_namespace, op_scope, true);
    }

    fn add_func_scope_edge(
        &mut self,
        func_namespace: NameSpaceID,
        func_scope: ScopeID,
        requires_return: bool,
    ) {
        // Register the function to the scope graph
        let global_scope_id = self
            .prog_ctx
            .scope_registry
            .get_global_scope_id(&func_namespace);
        // Add an edge from the global scope to the function
        self.comp_data
            .scope_graph
            .add_edge(global_scope_id, func_scope);
        // Mark the function scope as requires return
        self.comp_data
            .scope_graph
            .set_requires_return(func_scope, requires_return);
    }
}
