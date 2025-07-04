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
    Interpreter, Lexer, Parser, SemanticAnalyzer, SymbolInfo, SymbolKind, Value,
};
use std::collections::HashMap;

fn main() {
    divan::main();
}

#[divan::bench]
fn audio_shader_sample_processing() {
    let input = "
    input buffer in_buffer
    input float gain = 0.8
    output buffer out_buffer
    out_buffer = in_buffer * gain
    ";
    let lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let program = parser.parse();
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

    let mut ui_parameters = HashMap::new();
    ui_parameters.insert(
        "in_buffer".to_string(),
        SymbolInfo {
            name: "in_buffer".to_string(),
            kind: SymbolKind::Input,
            initial_value: None,
            range: None,
            value: Some(Value::from_buffer(vec![vec![0.15; 128]; 2])),
        },
    );
    ui_parameters.insert(
        "gain".to_string(),
        semantic_analyzer.input_table.get("gain").cloned().unwrap(),
    );

    let mut interpreter = Interpreter::new(program, 48000, 2, 0, 128);
    divan::black_box_drop(
        // Profile the execution time
        match interpreter.execute(ui_parameters) {
            Ok(_) => {}
            Err(_) => return,
        },
    );
}
