//
// Copyright 2025 Shuntaro Kasatani
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
    Parser, Program, SemanticAnalyzer, SymbolInfo, codegen::get_ir_type,
    compiler::codegen::Translator, executable::Executable,
};
use cranelift_codegen::{Context, ir::AbiParam};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use std::collections::HashMap;

pub struct Compiler {
    ctx: Context,
    module: JITModule,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let flags = [("enable_verifier", "false")];

        let builder = JITBuilder::with_flags(&flags, cranelift_module::default_libcall_names())?;
        // let builder = JITBuilder::new(cranelift_module::default_libcall_names())?;
        let module = JITModule::new(builder);
        let ctx = module.make_context();

        Ok(Compiler { ctx, module })
    }

    /// Compiles the given code and returns an `Executable` instance.
    pub fn compile(&mut self, code: &str) -> Result<Executable, Box<dyn std::error::Error>> {
        let mut program = Parser::new().parse(code)?;
        let mut semantic_analyzer = SemanticAnalyzer::new();
        program = semantic_analyzer.analyze(&program)?;
        let inputs = semantic_analyzer.get_inputs();
        let outputs = semantic_analyzer.get_outputs();

        let code_ptr = self.compile_program(&program, &inputs, &outputs)?;

        let func: unsafe extern "C" fn(*const u8, *mut u8) -> () =
            unsafe { std::mem::transmute(code_ptr) };

        // Calculate the outputs
        let output_types = outputs
            .iter()
            .map(|info| info.value_type.clone())
            .collect::<Vec<_>>();
        let output_size = output_types
            .iter()
            .map(|t| get_ir_type(t, &self.module).bytes())
            .sum::<u32>() as usize;
        let outputs = vec![0u8; output_size];

        Ok(Executable {
            func,
            outputs,
            output_types,
        })
    }

    /// Compiles the given program to machine code and returns a pointer to the compiled function.
    pub fn compile_program(
        &mut self,
        program: &Program,
        inputs: &Vec<SymbolInfo>,
        outputs: &Vec<SymbolInfo>,
    ) -> Result<*const u8, Box<dyn std::error::Error>> {
        self.translate(&program, &inputs, &outputs)?;

        println!("Translated function: {}", self.ctx.func.name);

        let func_name = "main";
        let id =
            self.module
                .declare_function(func_name, Linkage::Export, &self.ctx.func.signature)?;

        self.module.define_function(id, &mut self.ctx)?;
        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions()?;

        let func_addr = self.module.get_finalized_function(id);

        Ok(func_addr)
    }

    pub fn translate(
        &mut self,
        program: &Program,
        inputs: &Vec<SymbolInfo>,
        outputs: &Vec<SymbolInfo>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pointer_type = self.module.target_config().pointer_type();

        self.ctx
            .func
            .signature
            .params
            .push(AbiParam::new(pointer_type)); // input_ptr
        self.ctx
            .func
            .signature
            .params
            .push(AbiParam::new(pointer_type)); // output_ptr

        let mut builder_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut builder_ctx);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let mut translator = Translator::new(builder, HashMap::new(), entry_block);

        translator.setup_array_interface(inputs, outputs, &self.module);
        translator.define_builtin_functions(&mut self.module)?;

        for stmt in program.statements.iter() {
            translator.codegen_stmt(stmt, &self.module);
        }

        let output_names = &outputs.iter().map(|s| s.name.clone()).collect::<Vec<_>>();
        translator.builder.seal_all_blocks();

        translator.finalize_array_interface(output_names, &self.module);
        translator.builder.finalize();

        Ok(())
    }
}
