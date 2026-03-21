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
use cranelift::prelude::{AbiParam, FunctionBuilder, InstBuilder, types};
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
        // Append block params for function params
        builder.append_block_params_for_function_params(entry_block);

        // Switch to the entry block
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Get the block parameters
        let block_params = builder.block_params(entry_block).to_vec();
        let buffer_size = block_params[4];
        let translator_params = TranslatorParams {
            input_ptr_ptr: block_params[0],
            output_ptr_ptr: block_params[1],
            state_ptr_ptr: block_params[2],
            should_init: block_params[3],
        };

        // Create a FuncTranslator and translate the loop
        let mut translator =
            FuncTranslator::new(builder, &mut self.module, prog_ctx, builtin_registry);

        translator.create_loop(buffer_size, |translator, i, increment_block| {
            translator.translate(
                translator_params,
                Some(i),
                entry_point,
                blueprint,
                increment_block,
            );
        });

        // Add return statement after finishing the loop
        translator.builder.ins().return_(&[]);
        // Finalize the function
        translator.builder.finalize();
    }
}
