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

use crate::{TYPE_FLOAT, Translator};
use cranelift_codegen::ir::{self, InstBuilder};
use cranelift_jit::JITModule;
use cranelift_module::{Linkage, Module};

impl<'a> Translator<'a> {
    pub fn define_builtin_functions(
        &mut self,
        module: &mut JITModule,
    ) -> Result<(), cranelift_module::ModuleError> {
        let sinf = self.define_c_function("sinf", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        let cosf = self.define_c_function("cosf", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        let tanf = self.define_c_function("tanf", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        let asinf = self.define_c_function("asinf", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        let acosf = self.define_c_function("acosf", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        let atanf = self.define_c_function("atanf", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        let fabsf = self.define_c_function("fabs", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        // SGN
        // MIN
        // MAX
        // CLAMP
        let powf =
            self.define_c_function("powf", &[TYPE_FLOAT, TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        let sqrtf = self.define_c_function("sqrtf", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        let logf = self.define_c_function("logf", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        let log2f = self.define_c_function("log2f", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        let log10f = self.define_c_function("log10f", &[TYPE_FLOAT], Some(TYPE_FLOAT), module)?;
        // SAW
        // TRI
        // SQUARE
        // RAND
        // MIX
        // LERP
        // LOAD_TIME
        // LOAD_BEATS
        // PI
        // TIME
        // BEATS
        // SAMPLE_RATE
        // CHANNELS

        self.functions.insert("sin".to_string(), sinf);
        self.functions.insert("cos".to_string(), cosf);
        self.functions.insert("tan".to_string(), tanf);
        self.functions.insert("asin".to_string(), asinf);
        self.functions.insert("acos".to_string(), acosf);
        self.functions.insert("atan".to_string(), atanf);
        self.functions.insert("fabs".to_string(), fabsf);
        self.functions.insert("pow".to_string(), powf);
        self.functions.insert("sqrt".to_string(), sqrtf);
        self.functions.insert("log".to_string(), logf);
        self.functions.insert("log2".to_string(), log2f);
        self.functions.insert("log10".to_string(), log10f);
        // TODO: Add other functions as needed

        Ok(())
    }

    pub fn define_c_function(
        &mut self,
        name: &str,
        args: &[ir::Type],
        return_type: Option<ir::Type>,
        module: &mut JITModule,
    ) -> Result<ir::FuncRef, cranelift_module::ModuleError> {
        let mut func_sig = module.make_signature();
        for &arg in args {
            func_sig.params.push(ir::AbiParam::new(arg));
        }
        if let Some(ret_type) = return_type {
            func_sig.returns.push(ir::AbiParam::new(ret_type));
        }
        module
            .declare_function(name, Linkage::Import, &func_sig)
            .map(|func_id| module.declare_func_in_func(func_id, &mut self.builder.func))
    }

    pub fn call_function(&mut self, name: &str, args: Vec<ir::Value>) -> Vec<ir::Value> {
        let func = *self.functions.get(name).expect("Function not found");
        let call = self.builder.ins().call(func, &args);
        let results = self.builder.inst_results(call);
        results.into_iter().map(|v| v.to_owned()).collect()
    }
}
