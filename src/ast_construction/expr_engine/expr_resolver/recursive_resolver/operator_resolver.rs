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
    Expr, ExprKind, Range,
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

        // Capture the operator informations
        let lhs_name = op.lhs.name.clone();
        let rhs_name = op.rhs.name.clone();
        let op_scope = op.block.scope_id;
        let return_type = op.return_type;

        // Construct arguments
        let lhs_id = self.create_func_call_arg(lhs_name, lhs.clone(), &op_scope, lhs.range);
        let rhs_id = self.create_func_call_arg(rhs_name, rhs.clone(), &op_scope, rhs.range);

        Some(Expr::new(
            ExprKind::InfixOp {
                operator: op_id,
                lhs: lhs_id,
                rhs: rhs_id,
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

        // Capture the operator informations
        let operand_name = op.operand.name.clone();
        let op_scope = op.block.scope_id;
        let return_type = op.return_type;

        // Construct arguments
        let operand_id =
            self.create_func_call_arg(operand_name, operand.clone(), &op_scope, operand.range);

        Some(Expr::new(
            ExprKind::PrefixOp {
                operator: op_id,
                operand: operand_id,
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

        // Capture the operator informations
        let operand_name = op.operand.name.clone();
        let op_scope = op.block.scope_id;
        let return_type = op.return_type;

        // Construct arguments
        let operand_id =
            self.create_func_call_arg(operand_name, operand.clone(), &op_scope, operand.range);

        Some(Expr::new(
            ExprKind::PostfixOp {
                operator: op_id,
                operand: operand_id,
            },
            return_type,
            range,
        ))
    }
}
