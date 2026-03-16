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

use crate::{
    Expr, ExprKind, FuncCallArg, Range,
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
        let op_id = self.op_ctx.get_infix_id(InfixQueryRef {
            symbol: &symbol,
            lhs_type: &lhs.value_type,
            rhs_type: &rhs.value_type,
        })?;
        let op = self.op_ctx.get_infix_op(&op_id)?;

        // Add an operator call edge to the scope graph
        // This is used to detect recursion
        self.scope_graph
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
        let op_id = self.op_ctx.get_prefix_id(PrefixQueryRef {
            symbol: &symbol,
            operand_type: &operand.value_type,
        })?;
        let op = self.op_ctx.get_prefix_op(&op_id)?;

        // Add an operator call edge to the scope graph
        // This is used to detect recursion
        self.scope_graph
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
        let op_id = self.op_ctx.get_postfix_id(PostfixQueryRef {
            symbol: &symbol,
            operand_type: &operand.value_type,
        })?;
        let op = self.op_ctx.get_postfix_op(&op_id)?;

        // Add an operator call edge to the scope graph
        // This is used to detect recursion
        self.scope_graph
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
