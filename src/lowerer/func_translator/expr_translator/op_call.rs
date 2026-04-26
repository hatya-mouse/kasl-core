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

use std::slice::from_ref;

use kasl_ir::Value;

use crate::{
    ast_nodes::{FuncCallArg, OperatorID},
    lowerer::func_translator::FuncTranslator,
};

impl FuncTranslator<'_> {
    pub(super) fn translate_infix_op_expr(
        &mut self,
        op_id: &OperatorID,
        lhs: &FuncCallArg,
        rhs: &FuncCallArg,
    ) -> Value {
        // Get the operator function block
        let op = &self.prog_ctx.op_ctx.get_infix_func(op_id).unwrap();
        self.call_func(&op.block, &[lhs.clone(), rhs.clone()], &op.return_type)
            .unwrap()
    }

    pub(super) fn translate_prefix_op_expr(
        &mut self,
        op_id: &OperatorID,
        operand: &FuncCallArg,
    ) -> Value {
        // Get the operator function block
        let op = &self.prog_ctx.op_ctx.get_prefix_func(op_id).unwrap();
        self.call_func(&op.block, from_ref(operand), &op.return_type)
            .unwrap()
    }

    pub(super) fn translate_postfix_op_expr(
        &mut self,
        op_id: &OperatorID,
        operand: &FuncCallArg,
    ) -> Value {
        // Get the operator function block
        let op = &self.prog_ctx.op_ctx.get_postfix_func(op_id).unwrap();
        self.call_func(&op.block, from_ref(operand), &op.return_type)
            .unwrap()
    }
}
