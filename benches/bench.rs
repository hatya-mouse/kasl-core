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

use knodiq_audio_shader::{
    Compiler, Interpreter, Parser, SemanticAnalyzer, SymbolInfo, SymbolKind, compile,
    run::Executable,
};
use knodiq_engine::{Type, Value};

fn main() {
    divan::main();
}

#[divan::bench]
fn audio_shader_sample_processing() {
    let input = "
    input [[float]] in_buffer
    input float gain = 0.8
    output [[float]] out_buffer
    out_buffer = in_buffer * gain
    ";
    let parser = Parser::new();
    let program = parser.parse(&input);
    match &program {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
        }
    }

    let program = program.unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();
    match semantic_analyzer
        .analyze(&program)
        .map_err(|e| format!("{:?}", e))
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Semantic analysis error: {}", e);
            return;
        }
    }

    let mut ui_parameters = Vec::new();
    ui_parameters.push(SymbolInfo {
        name: "in_buffer".to_string(),
        kind: SymbolKind::Input,
        value_type: Type::Array(Box::new(Type::Array(Box::new(Type::Float)))),
        value: Some(Value::from_buffer(vec![vec![0.15; 128]; 2])),
    });
    ui_parameters.push(semantic_analyzer.input_table[1].clone());

    let mut interpreter = Interpreter::new(program, 48000, 24000.0, 2, 0, 128);
    divan::black_box_drop(
        // Profile the execution time
        match interpreter.execute(ui_parameters) {
            Ok(_) => {}
            Err(_) => return,
        },
    );
}

#[divan::bench(sample_count = 1000, sample_size = 10)]
fn interpreter() {
    let code = "
    input float in_buffer
    output float out_buffer
    output float powered
    var gain = 1.0
    var result = 0.0

    result = in_buffer * gain
    out_buffer = result + 1.25
    powered = in_buffer * in_buffer
    ";

    let parser = Parser::new();
    let program = parser.parse(&code).unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let mut interpreter = Interpreter::new(program, 48000, 24000.0, 2, 0, 2);

    let mut input_table = analyzer.input_table.clone();
    input_table[0].value = Some(Value::Float(2.0));

    divan::black_box_drop(match interpreter.execute(input_table) {
        Ok(_) => {}
        Err(_) => return,
    });
}

#[divan::bench]
fn jit_normal() {
    // Exclude compilation time
    static EXEC: std::sync::OnceLock<std::sync::Mutex<Executable>> = std::sync::OnceLock::new();

    let exec = EXEC.get_or_init(|| std::sync::Mutex::new(setup_jit()));

    // Measure execution time
    let inputs = vec![Value::Float(2.0)];
    divan::black_box_drop(exec.lock().unwrap().run(inputs).unwrap());
}

#[divan::bench]
fn jit_direct() {
    static EXEC: std::sync::OnceLock<std::sync::Mutex<Executable>> = std::sync::OnceLock::new();

    let exec = EXEC.get_or_init(|| std::sync::Mutex::new(setup_jit()));

    let mut exec = exec.lock().unwrap();

    unsafe {
        (exec.func)(&2u8, 1, exec.outputs.as_mut_ptr(), exec.outputs.len());
    }
    divan::black_box(exec.outputs[0]);
}

#[divan::bench]
fn jit_raw_function_call() {
    static FUNC: std::sync::OnceLock<unsafe extern "C" fn(*const u8, usize, *mut f32, usize)> =
        std::sync::OnceLock::new();

    let func = FUNC.get_or_init(|| {
        let exec = setup_jit();
        exec.func
    });

    let input = 2u8;
    let mut output = [0.0f32; 3]; // output count

    unsafe {
        func(&input, 1, output.as_mut_ptr(), output.len());
    }

    divan::black_box(output[0]);
}

#[divan::bench(sample_count = 1000, sample_size = 10)]
fn jit() {
    static EXEC: std::sync::OnceLock<std::sync::Mutex<Executable>> = std::sync::OnceLock::new();

    let exec = EXEC.get_or_init(|| std::sync::Mutex::new(setup_jit()));

    let inputs = vec![Value::Float(2.0)];

    let start = std::time::Instant::now();
    let result = exec.lock().unwrap().run(inputs).unwrap();
    let duration = start.elapsed();

    if duration.as_micros() > 100 {
        eprintln!("Slow execution detected: {:?}", duration);
    }

    divan::black_box_drop(result);
}

fn setup_jit() -> Executable {
    let code = "
    input float in_buffer
    output float out_buffer
    output float powered
    var gain = 1.0
    var result = 0.0

    result = in_buffer * gain
    out_buffer = result + 1.25
    powered = in_buffer * in_buffer
    ";

    let mut compiler = Compiler::new().unwrap();
    compile(&mut compiler, &code).unwrap()
}
