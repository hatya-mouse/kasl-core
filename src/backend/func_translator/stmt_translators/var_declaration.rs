use crate::{VariableID, backend::func_translator::FuncTranslator, type_registry::ResolvedType};
use cranelift::prelude::Variable;

impl FuncTranslator<'_> {
    pub fn declare_var(&mut self, var_id: VariableID, var_type: &ResolvedType) -> Variable {
        // Convert the ResolvedType into Type
        let ir_type = self.type_converter.convert(var_type);

        // Declare the var and store it to the scope registry
        self.scope_registry
            .add_var(var_id, self.builder.declare_var(ir_type))
    }
}
