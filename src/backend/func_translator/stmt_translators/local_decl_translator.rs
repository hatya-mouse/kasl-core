use crate::{VariableID, backend::func_translator::FuncTranslator};

impl FuncTranslator<'_> {
    pub fn translate_local_var(&mut self, var_id: &VariableID) {
        // Get the ScopeVar for the id
        let local_var = self.prog_ctx.scope_registry.get_var(var_id).unwrap();
        // Declare the variable
        let var = self.declare_var(*var_id, &local_var.value_type);

        // Translate the expression and store the value
        // The variable is a local variable so it should be safe to unwrap the value
        let value = self.translate_expr(local_var.expect_def_val()).unwrap();
        self.builder.def_var(var, value);
    }

    pub fn translate_local_const(&mut self, var_id: &VariableID) {
        // Get the ScopeVar
        let local_const = self.prog_ctx.scope_registry.get_var(var_id).unwrap();
        // Declare the variable
        let var = self.declare_var(*var_id, &local_const.value_type);

        // Translate the expression and store the value
        // The variable is a local constant so it should be safe to unwrap the value
        let value = self.translate_expr(local_const.expect_def_val()).unwrap();
        self.builder.def_var(var, value);
    }
}
