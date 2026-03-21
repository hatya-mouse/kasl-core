use crate::{VariableID, backend::func_translator::FuncTranslator};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub(super) fn translate_identifier(&mut self, var_id: &VariableID) -> ir::Value {
        // Get the variable and get the value
        let var = self.scope_registry.get_var(var_id);
        self.builder.use_var(var)
    }
}
