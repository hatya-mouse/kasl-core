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

use crate::{VariableID, backend::func_translator::FuncTranslator, type_registry::ResolvedType};
use cranelift::prelude::Variable;

impl FuncTranslator<'_> {
    pub fn declare_var(&mut self, var_id: VariableID, var_type: &ResolvedType) -> Variable {
        // Convert the ResolvedType into Type
        let ir_type = self.type_converter.convert(var_type);

        // Declare the var and store it to the map
        *self
            .variables
            .entry(var_id)
            .or_insert_with(|| self.builder.declare_var(ir_type))
    }
}
