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

use crate::{FuncCallArg, OperatorID, backend::func_translator::FuncTranslator};
use cranelift_codegen::ir;
use std::slice;

impl FuncTranslator<'_> {
    pub fn translate_infix_op_expr(
        &mut self,
        op_id: &OperatorID,
        lhs: &FuncCallArg,
        rhs: &FuncCallArg,
    ) -> ir::Value {
        // Get the operator function block
        let op_block = &self.comp_state.op_ctx.get_infix_op(op_id).unwrap().block;
        self.call_func(op_block, &[lhs.clone(), rhs.clone()])
    }

    pub fn translate_prefix_op_expr(
        &mut self,
        op_id: &OperatorID,
        operand: &FuncCallArg,
    ) -> ir::Value {
        // Get the operator function block
        let op_block = &self.comp_state.op_ctx.get_infix_op(op_id).unwrap().block;
        self.call_func(op_block, slice::from_ref(operand))
    }

    pub fn translate_postfix_op_expr(
        &mut self,
        op_id: &OperatorID,
        operand: &FuncCallArg,
    ) -> ir::Value {
        // Get the operator function block
        let op_block = &self.comp_state.op_ctx.get_infix_op(op_id).unwrap().block;
        self.call_func(op_block, slice::from_ref(operand))
    }
}
