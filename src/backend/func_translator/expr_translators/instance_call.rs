use crate::{FuncCallArg, FunctionID, backend::func_translator::FuncTranslator};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub(super) fn translate_instance_call_expr(
        &mut self,
        id: &FunctionID,
        args: &[FuncCallArg],
    ) -> Option<ir::Value> {
        // Call the function and get the result
        let func = self.prog_ctx.func_ctx.get_func(id).unwrap();
        self.call_func(&func.block, args.as_ref(), &func.return_type)
    }
}
