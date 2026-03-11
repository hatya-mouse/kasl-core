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

use crate::{
    FuncCallArg, FunctionID, backend::func_translator::FuncTranslator, symbol_table,
    type_registry::ResolvedType,
};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_func_call_expr(
        &mut self,
        func_id: &FunctionID,
        args: &[FuncCallArg],
    ) -> Option<ir::Value> {
        // Get the function block
        let func = &self.comp_state.func_ctx.get_func(func_id).unwrap();
        self.call_func(&func.block, args, &func.return_type)
    }

    pub fn call_func(
        &mut self,
        block: &symbol_table::Block,
        args: &[FuncCallArg],
        expected_return_type: &Option<ResolvedType>,
    ) -> Option<ir::Value> {
        // Define the argument as variables
        for arg in args {
            let arg_var = self.declare_var(arg.var_id, &arg.value.value_type);
            let translated_val = self.translate_expr(&arg.value).unwrap();
            self.builder.def_var(arg_var, translated_val);
        }

        // Create a return block and set it as the return block
        let func_return_block = self.builder.create_block();

        // Get the return type
        if let Some(return_type) = expected_return_type
            .as_ref()
            .map(|ty| self.type_converter.convert(ty))
        {
            self.builder
                .append_block_param(func_return_block, return_type);
        }

        // Translate the block
        self.translate_block(block, func_return_block);

        // Add some arguments to the return block
        self.builder.switch_to_block(func_return_block);
        self.builder.seal_block(func_return_block);

        // Retrieve the return value
        if expected_return_type.is_some() {
            Some(self.builder.block_params(func_return_block)[0])
        } else {
            None
        }
    }
}
