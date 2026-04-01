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
    ast::{FuncCallArg, FunctionID, Statement, symbol_table, type_registry::ResolvedType},
    lowerer::func_translator::{FuncTranslator, type_converter::convert_type},
};
use kasl_ir::{InstBuilder, Value};

impl FuncTranslator<'_> {
    pub(super) fn translate_func_call_expr(
        &mut self,
        func_id: &FunctionID,
        args: &[FuncCallArg],
    ) -> Option<Value> {
        // Get the function block
        let func = &self.prog_ctx.func_ctx.get_func(func_id).unwrap();
        self.call_func(&func.block, args, &func.return_type)
    }

    pub(super) fn call_func(
        &mut self,
        block: &symbol_table::Block,
        args: &[FuncCallArg],
        expected_return_type: &ResolvedType,
    ) -> Option<Value> {
        // Push a new scope
        self.scope_registry.push_deepest();

        // Define the arguments as variables
        for arg in args {
            let arg_var = self.declare_var(arg.var_id, &arg.value.value_type);
            let translated_val = self.translate_expr(&arg.value).unwrap();
            self.builder.assign(arg_var, translated_val);
        }

        // Build the function body
        // If the body of the function is a single return statement, optimize it by returning the value directly
        if block.body.len() == 1
            && let Statement::Return { value } = &block.body[0]
        {
            let return_value = value.as_ref().and_then(|v| self.translate_expr(v));
            self.scope_registry.pop_deepest();
            return return_value;
        }

        // Get the return types
        let mut return_types = Vec::new();
        if !expected_return_type.is_void() {
            let return_type = convert_type(expected_return_type);
            return_types.push(return_type);
        }

        // Create a function return block and set it as the return block
        let func_return_block = self.builder.create_block(&return_types);

        // Translate the block
        let has_return = self.translate_block(block, func_return_block);
        if !has_return {
            self.builder.jump(func_return_block, &[]);
        }

        // Pop the scope
        self.scope_registry.pop_deepest();

        // Switch to the return block and get the return value
        self.builder.switch_to_block(func_return_block);
        if !expected_return_type.is_void() {
            Some(self.builder.get_block_args(func_return_block)[0])
        } else {
            None
        }
    }
}
