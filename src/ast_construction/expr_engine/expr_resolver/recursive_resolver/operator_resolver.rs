use crate::{
    Expr, ExprKind, FuncCallArg, Range,
    error::Ph,
    expr_engine::ExpressionResolver,
    symbol_table::{InfixQueryRef, PostfixQueryRef, PrefixQueryRef, UnresolvedExpr},
};

impl ExpressionResolver<'_> {
    pub fn resolve_infix_op(
        &mut self,
        symbol: String,
        lhs: UnresolvedExpr,
        rhs: UnresolvedExpr,
        range: Range,
    ) -> Option<Expr> {
        // Resolve the types of the operands recursively
        let lhs = self.resolve_recursively(lhs)?;
        let rhs = self.resolve_recursively(rhs)?;

        // Get a reference to the actual operator
        let Some(op_id) = self.prog_ctx.op_ctx.get_infix_id(InfixQueryRef {
            symbol: &symbol,
            lhs_type: &lhs.value_type,
            rhs_type: &rhs.value_type,
        }) else {
            self.ec.infix_op_not_found(
                range,
                Ph::ExprEngine,
                &symbol,
                &self.prog_ctx.type_registry.format_type(&lhs.value_type),
                &self.prog_ctx.type_registry.format_type(&rhs.value_type),
            );
            return None;
        };
        let op = self.prog_ctx.op_ctx.get_infix_func(&op_id)?;

        // Add an operator call edge to the scope graph
        // This is used to detect recursion
        self.comp_data
            .scope_graph
            .add_edge(self.current_scope, op.block.scope_id);

        // Construct arguments
        let lhs_arg = FuncCallArg {
            var_id: op.lhs.var_id,
            value: lhs.clone(),
            range: lhs.range,
        };
        let rhs_arg = FuncCallArg {
            var_id: op.rhs.var_id,
            value: rhs.clone(),
            range: rhs.range,
        };

        // Get the return type of the operator
        let return_type = op.return_type;

        Some(Expr::new(
            ExprKind::InfixOp {
                operator: op_id,
                lhs: Box::new(lhs_arg),
                rhs: Box::new(rhs_arg),
            },
            return_type,
            range,
        ))
    }

    pub fn resolve_prefix_op(
        &mut self,
        symbol: String,
        operand: UnresolvedExpr,
        range: Range,
    ) -> Option<Expr> {
        // Resolve the type of the operand
        let operand = self.resolve_recursively(operand)?;

        // Get a reference to the actual operator
        let Some(op_id) = self.prog_ctx.op_ctx.get_prefix_id(PrefixQueryRef {
            symbol: &symbol,
            operand_type: &operand.value_type,
        }) else {
            self.ec.prefix_op_not_found(
                range,
                Ph::ExprEngine,
                &symbol,
                &self.prog_ctx.type_registry.format_type(&operand.value_type),
            );
            return None;
        };
        let op = self.prog_ctx.op_ctx.get_prefix_func(&op_id)?;

        // Add an operator call edge to the scope graph
        // This is used to detect recursion
        self.comp_data
            .scope_graph
            .add_edge(self.current_scope, op.block.scope_id);

        // Construct arguments
        let operand_arg = FuncCallArg {
            var_id: op.operand.var_id,
            value: operand.clone(),
            range: operand.range,
        };

        // Get the return type of the operator
        let return_type = op.return_type;

        Some(Expr::new(
            ExprKind::PrefixOp {
                operator: op_id,
                operand: Box::new(operand_arg),
            },
            return_type,
            range,
        ))
    }

    pub fn resolve_postfix_op(
        &mut self,
        symbol: String,
        operand: UnresolvedExpr,
        range: Range,
    ) -> Option<Expr> {
        // Resolve the type of the operand
        let operand = self.resolve_recursively(operand)?;

        // Get a reference to the actual operator
        let Some(op_id) = self.prog_ctx.op_ctx.get_postfix_id(PostfixQueryRef {
            symbol: &symbol,
            operand_type: &operand.value_type,
        }) else {
            self.ec.postfix_op_not_found(
                range,
                Ph::ExprEngine,
                &symbol,
                &self.prog_ctx.type_registry.format_type(&operand.value_type),
            );
            return None;
        };
        let op = self.prog_ctx.op_ctx.get_postfix_func(&op_id)?;

        // Add an operator call edge to the scope graph
        // This is used to detect recursion
        self.comp_data
            .scope_graph
            .add_edge(self.current_scope, op.block.scope_id);

        // Construct arguments
        let operand_arg = FuncCallArg {
            var_id: op.operand.var_id,
            value: operand.clone(),
            range: operand.range,
        };

        // Get the return type of the operator
        let return_type = op.return_type;

        Some(Expr::new(
            ExprKind::PostfixOp {
                operator: op_id,
                operand: Box::new(operand_arg),
            },
            return_type,
            range,
        ))
    }
}
