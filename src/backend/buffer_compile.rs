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
    FunctionID,
    backend::{
        Backend,
        func_translator::{FuncTranslator, TranslatorParams},
    },
    builtin::BuiltinRegistry,
    compilation_data::ProgramContext,
    scope_manager::IOBlueprint,
};
use cranelift::prelude::{AbiParam, FunctionBuilder, InstBuilder, IntCC, types};
use cranelift_codegen::{settings, verify_function};
use cranelift_module::{Linkage, Module};

impl Backend {
    /// Compiles the program that processes a buffer.
    /// Signature:
    /// ```
    /// fn(input, output, state, should_init, buffer_size)
    /// ```
    pub fn compile_buffer(
        &mut self,
        prog_ctx: &ProgramContext,
        builtin_registry: &BuiltinRegistry,
        blueprint: &IOBlueprint,
        entry_point: &FunctionID,
    ) -> Result<*const u8, String> {
        self.translate_buffer(prog_ctx, builtin_registry, blueprint, entry_point);

        // Verify the function
        let verifier_flags = settings::Flags::new(settings::builder());
        verify_function(&self.ctx.func, &verifier_flags).map_err(|e| e.to_string())?;

        let id = self
            .module
            .declare_function("main", Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;
        self.module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())?;

        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions().unwrap();

        let code = self.module.get_finalized_function(id);
        Ok(code)
    }

    pub fn translate_buffer(
        &mut self,
        prog_ctx: &ProgramContext,
        builtin_registry: &BuiltinRegistry,
        blueprint: &IOBlueprint,
        entry_point: &FunctionID,
    ) {
        // Add parameter for the input and output pointers
        let pointer_type = self.module.target_config().pointer_type();
        self.ctx.func.signature.params.extend(&[
            AbiParam::new(pointer_type), // Input pointers
            AbiParam::new(pointer_type), // Output pointers
            AbiParam::new(pointer_type), // State pointers
            AbiParam::new(types::I8),    // Should init
            AbiParam::new(types::I32),   // Size of the buffer
        ]);

        // Create a function builder
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);

        // Create needed blocks
        let entry_block = builder.create_block();
        let body_block = builder.create_block();
        let increment_block = builder.create_block();
        let return_block = builder.create_block();
        let loop_header_block = builder.create_block();

        // Append block params for function params
        builder.append_block_params_for_function_params(entry_block);
        // Switch to the entry block
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);
        // Get the buffer size

        // Get the pointer to the pointer array and the buffer size
        let block_params = builder.block_params(entry_block);
        let translator_params = TranslatorParams {
            input_ptr_ptr: block_params[0],
            output_ptr_ptr: block_params[1],
            state_ptr_ptr: block_params[2],
            should_init: block_params[3],
        };
        let buffer_size = block_params[4];

        // Create a variable to store the current loop index
        let i = builder.declare_var(types::I32);
        let zero_val = builder.ins().iconst(types::I32, 0);
        builder.def_var(i, zero_val);

        // Jump to the body or the return block depending on the index
        builder.ins().jump(loop_header_block, &[]);
        builder.switch_to_block(loop_header_block);

        let i_val = builder.use_var(i);
        let continue_loop = builder
            .ins()
            .icmp(IntCC::UnsignedLessThan, i_val, buffer_size);
        builder
            .ins()
            .brif(continue_loop, body_block, &[], return_block, &[]);

        // Create a body block and the return block
        builder.switch_to_block(body_block);
        builder.seal_block(body_block);

        // Create a FuncTranslator and translate the function
        let mut translator = FuncTranslator::new(builder, &self.module, prog_ctx, builtin_registry);
        translator.translate(
            translator_params,
            Some(i_val),
            entry_point,
            blueprint,
            increment_block,
        );

        // Add jump instruction to the increment block at the end of the body
        translator.builder.ins().jump(increment_block, &[]);
        translator.builder.switch_to_block(increment_block);
        translator.builder.seal_block(increment_block);

        // Increment the index
        let one_val = translator.builder.ins().iconst(types::I32, 1);
        let next_i = translator.builder.ins().iadd(i_val, one_val);
        translator.builder.def_var(i, next_i);

        // Add jump instruction to the loop header block
        translator.builder.ins().jump(loop_header_block, &[]);
        translator.builder.seal_block(loop_header_block);

        // Add return instruction to the return block
        translator.builder.switch_to_block(return_block);
        translator.builder.seal_block(return_block);

        translator.builder.ins().return_(&[]);

        // Finalize the function
        translator.builder.finalize();
    }
}
