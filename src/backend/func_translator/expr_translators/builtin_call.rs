use crate::{Expr, backend::func_translator::FuncTranslator, builtin::BuiltinFuncID};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub(super) fn translate_builtin_func_call(
        &mut self,
        func_id: &BuiltinFuncID,
        args: &[Expr],
    ) -> ir::Value {
        // Translate the expressions
        let mut translated_args = Vec::new();
        for arg in args {
            let translated_val = self.translate_expr(arg).unwrap();
            translated_args.push(translated_val);
        }

        // Get a reference to the function
        let func = &self.builtin_registry.get_func_by_id(func_id).unwrap();

        // Translate the function
        (func.translator)(&mut self.builder, &translated_args)
    }
}
