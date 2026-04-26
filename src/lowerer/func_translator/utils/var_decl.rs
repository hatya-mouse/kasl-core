//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{
    ast_nodes::{VariableID, type_registry::ResolvedType},
    lowerer::func_translator::{FuncTranslator, type_converter::convert_type},
};
use kasl_ir::Variable;

impl FuncTranslator<'_> {
    pub fn declare_var(&mut self, var_id: VariableID, var_type: &ResolvedType) -> Variable {
        // Convert the ResolvedType into Type
        let ir_type = convert_type(var_type);

        // Declare the var and store it to the scope registry
        self.scope_registry
            .add_var(var_id, self.builder.create_var(ir_type))
    }
}
