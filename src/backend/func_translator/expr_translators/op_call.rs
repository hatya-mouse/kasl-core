use crate::{FuncCallArg, OperatorID, backend::func_translator::FuncTranslator};
use cranelift_codegen::ir;
use std::slice;

impl FuncTranslator<'_> {
    pub(super) fn translate_infix_op_expr(
        &mut self,
        op_id: &OperatorID,
        lhs: &FuncCallArg,
        rhs: &FuncCallArg,
    ) -> ir::Value {
        // Get the operator function block
        let op = &self.prog_ctx.op_ctx.get_infix_func(op_id).unwrap();
        self.call_func(&op.block, &[lhs.clone(), rhs.clone()], &op.return_type)
            .unwrap()
    }

    pub(super) fn translate_prefix_op_expr(
        &mut self,
        op_id: &OperatorID,
        operand: &FuncCallArg,
    ) -> ir::Value {
        // Get the operator function block
        let op = &self.prog_ctx.op_ctx.get_prefix_func(op_id).unwrap();
        self.call_func(&op.block, slice::from_ref(operand), &op.return_type)
            .unwrap()
    }

    pub(super) fn translate_postfix_op_expr(
        &mut self,
        op_id: &OperatorID,
        operand: &FuncCallArg,
    ) -> ir::Value {
        // Get the operator function block
        let op = &self.prog_ctx.op_ctx.get_postfix_func(op_id).unwrap();
        self.call_func(&op.block, slice::from_ref(operand), &op.return_type)
            .unwrap()
    }
}
