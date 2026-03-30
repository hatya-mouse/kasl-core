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

mod func_translator;

use crate::{
    ast::{FunctionID, compilation_data::ProgramContext, scope_manager::IOBlueprint},
    lowerer::func_translator::FuncTranslator,
};
use kasl_ir::ir::{IRBuilder, IRType, InstBuilder, Value};

pub struct TranslatorParams {
    pub input_ptr_ptr: Value,
    pub output_ptr_ptr: Value,
    pub state_ptr_ptr: Value,
    pub should_init: Value,
}

#[derive(Default)]
pub struct Lowerer;

impl Lowerer {
    /// Lower the given program context to KASL-IR.
    pub fn lower(
        &self,
        prog_ctx: &ProgramContext,
        blueprint: &IOBlueprint,
        entry_point: &FunctionID,
    ) {
        // Create a ir builder
        let mut builder = IRBuilder::default();

        // Create an entry block and switch to the block
        // Add parameter for the input and output pointers
        // 1: input pointer
        // 2: output pointer
        // 3: state pointer
        // 4: whether to initialize the states
        let entry_block =
            builder.create_block(&[IRType::Ptr, IRType::Ptr, IRType::Ptr, IRType::I8]);
        builder.switch_to_block(entry_block);

        // Set the block as the entry block of the function
        builder.set_entry_block(entry_block);

        // Get the entry block parameters
        let Some(block_params) = builder.get_block_args(entry_block) else {
            return;
        };
        let translator_params = TranslatorParams {
            input_ptr_ptr: block_params[0],
            output_ptr_ptr: block_params[1],
            state_ptr_ptr: block_params[2],
            should_init: block_params[3],
        };

        // Create a return block
        let return_block = builder.create_block(&[]);

        // Lower the program context to KASL-IR
        let mut translator = FuncTranslator::new(builder, prog_ctx);
        translator.translate(
            translator_params,
            None,
            entry_point,
            blueprint,
            return_block,
        );

        // Add jump instruction
        translator.builder.jump(return_block, &[]);

        // Add return instruction to the return block
        translator.builder.switch_to_block(return_block);
        translator.builder._return(&[]);
    }
}
