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

use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir::immediates::Offset32;

use crate::{Expr, backend::func_translator::FuncTranslator, symbol_table::LValue};

impl FuncTranslator<'_> {
    pub fn translate_assign(&mut self, target: &LValue, value: &Expr) {
        // Translate the RHS value
        let rhs_value = self.translate_expr(value);

        // Get the Variable and the ScopeVar
        let var = self.variables[&target.var_id];

        // Set the value to the variable depending on the value type
        if target.is_field {
            let addr = self.builder.use_var(var);
            self.builder.ins().store(
                MemFlags::new(),
                rhs_value,
                addr,
                Offset32::new(target.offset),
            );
        } else {
            self.builder.def_var(var, rhs_value);
        }
    }
}
