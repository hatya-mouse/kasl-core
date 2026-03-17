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

use crate::{VariableID, backend::func_translator::FuncTranslator};

impl FuncTranslator<'_> {
    pub fn translate_local_var(&mut self, var_id: &VariableID) {
        // Get the variable
        let local_var = self.prog_ctx.scope_registry.get_var(var_id).unwrap();
        // Get the Variable
        let var = self.variables[var_id];

        // Translate the expression and store the value
        // The variable is a local variable so it should be safe to unwrap the value
        let value = self.translate_expr(local_var.expect_def_val());
        self.builder.def_var(var, value);
    }

    pub fn translate_local_const(&mut self, var_id: &VariableID) {
        // Get the ScopeVar
        let local_const = self.prog_ctx.scope_registry.get_var(var_id).unwrap();
        // Get the Variable
        let var = self.variables[var_id];

        // Translate the expression and store the value
        // The variable is a local constant so it should be safe to unwrap the value
        let value = self.translate_expr(local_const.expect_def_val());
        self.builder.def_var(var, value);
    }
}
