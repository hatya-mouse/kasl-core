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

use std::collections::HashMap;

use crate::{Program, SemanticAnalyzer, codegen::get_type, compiler::codegen::Translator};
use cranelift_codegen::{
    Context,
    ir::{self, AbiParam, InstBuilder},
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Module;

pub struct Compiler {
    ctx: Context,
    module: JITModule,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names())?;
        let module = JITModule::new(builder);

        Ok(Compiler {
            ctx: module.make_context(),
            module,
        })
    }

    pub fn execute(
        &mut self,
        program: &Program,
    ) -> Result<Vec<ir::Value>, Box<dyn std::error::Error>> {
        let mut semantic_analyzer = SemanticAnalyzer::new();
        let program = semantic_analyzer.analyze(program)?;

        let inputs = semantic_analyzer.get_input_table();
        let outputs = semantic_analyzer.get_output_table();
        let mut input_names = Vec::new();
        let mut output_names = Vec::new();

        for input in inputs {
            let var_type = get_type(input.1.value_type, &self.module);
            self.ctx.func.signature.params.push(AbiParam::new(var_type));
            input_names.push(input.1.name.clone());
        }

        for output in outputs {
            let var_type = get_type(output.1.value_type, &self.module);
            self.ctx
                .func
                .signature
                .returns
                .push(AbiParam::new(var_type));
            output_names.push(output.1.name.clone());
        }

        let mut builder_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut builder_ctx);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let return_block = builder.create_block();
        builder.append_block_params_for_function_returns(return_block);

        let mut translator = Translator::new(builder, HashMap::new(), entry_block, return_block);
        for stmt in program.statements.iter() {
            translator.codegen_stmt(&input_names, &output_names, stmt, &self.module);
        }

        let return_vals = translator.get_returns();
        translator.builder.ins().return_(&return_vals);
        translator.builder.finalize();

        Ok(return_vals)
    }
}
