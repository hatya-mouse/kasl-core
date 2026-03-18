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
    FunctionID, Statement, VariableID, backend::func_translator::FuncTranslator, symbol_table,
    type_registry::ResolvedType,
};
use cranelift::prelude::InstBuilder;
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn translate_func_call_expr(
        &mut self,
        func_id: &FunctionID,
        args: &[VariableID],
    ) -> ir::Value {
        // Get the function block
        let func = &self.prog_ctx.func_ctx.get_func(func_id).unwrap();
        self.call_func(&func.block, args, &func.return_type)
            .unwrap()
    }

    pub fn call_func(
        &mut self,
        block: &symbol_table::Block,
        arg_ids: &[VariableID],
        expected_return_type: &ResolvedType,
    ) -> Option<ir::Value> {
        // Define the argument as variables
        for arg_id in arg_ids {
            // Get the argument variable which is unique for the function call
            let scope_var = self.prog_ctx.scope_registry.get_var(arg_id).unwrap();

            // Declare the argument variable in the IR
            let ir_var = self.declare_var(*arg_id, &scope_var.value_type);
            // Translate the argument value and define it
            let translated_val = self.translate_expr(scope_var.def_val.as_ref().unwrap());
            // Assign the translated value to the argument variable
            self.builder.def_var(ir_var, translated_val);
        }

        // If the body of the function is a single return statement, we can optimize by returning the value directly
        if block.body.len() == 1
            && let Statement::Return { value } = &block.body[0]
        {
            if let Some(value) = value {
                return Some(self.translate_expr(value));
            } else {
                return None;
            }
        }

        // Create a return block and set it as the return block
        let func_return_block = self.builder.create_block();

        // Get the return type
        if !expected_return_type.is_void() {
            let return_type = self.type_converter.convert(expected_return_type);
            self.builder
                .append_block_param(func_return_block, return_type);
        }

        // Translate the block
        let has_return = self.translate_block(block, func_return_block);
        if !has_return {
            self.builder.ins().jump(func_return_block, &[]);
        }

        // Add some arguments to the return block
        self.builder.switch_to_block(func_return_block);
        self.builder.seal_block(func_return_block);

        // Retrieve the return value
        if !expected_return_type.is_void() {
            Some(self.builder.block_params(func_return_block)[0])
        } else {
            None
        }
    }
}
