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

use crate::Compiler;
use knodiq_engine::Value;

pub struct Executable {
    pub func: unsafe extern "C" fn(*const f32, usize, *mut f32, usize) -> (),
    pub outputs: Vec<f32>,
}

pub fn compile(
    compiler: &mut Compiler,
    code: &str,
) -> Result<Executable, Box<dyn std::error::Error>> {
    let code_ptr = compiler.compile(code)?;

    let func: unsafe extern "C" fn(*const f32, usize, *mut f32, usize) -> () =
        unsafe { std::mem::transmute(code_ptr) };

    // Get the output count
    let output_count = get_output_count(code)?;
    let outputs = vec![0.0f32; output_count];

    Ok(Executable { func, outputs })
}

pub fn run_fn(
    exec: &mut Executable,
    inputs: Vec<Value>,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    // Convert inputs to a flat vector of f32
    let mut input_floats = Vec::new();
    for input in &inputs {
        flatten_value_to_floats(input, &mut input_floats);
    }

    // Execute the function
    unsafe {
        (exec.func)(
            input_floats.as_ptr(),
            input_floats.len(),
            exec.outputs.as_mut_ptr(),
            exec.outputs.len(),
        );
    }

    // Convert results to Value
    let result: Vec<Value> = exec.outputs.iter().map(|&f| Value::Float(f)).collect();

    Ok(result)
}

fn flatten_value_to_floats(value: &Value, output: &mut Vec<f32>) {
    match value {
        Value::Int(i) => output.push(*i as f32),
        Value::Float(f) => output.push(*f),
        Value::Array(arr) => {
            for elem in arr {
                flatten_value_to_floats(elem, output);
            }
        }
    }
}

fn get_output_count(code: &str) -> Result<usize, Box<dyn std::error::Error>> {
    use crate::{Parser, SemanticAnalyzer};

    let parser = Parser::new();
    let program = parser.parse(code)?;
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program)?;

    let outputs = analyzer.get_outputs();

    // Count the total number of output values
    let mut total_count = 0;
    for output_info in outputs {
        total_count += match &output_info.value {
            Some(Value::Array(vec)) => vec.len(),
            _ => 1,
        };
    }

    Ok(total_count)
}
