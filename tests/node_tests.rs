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
    AudioShaderNode, Lexer, Node, Parser, SemanticAnalyzer, SymbolKind, Value,
}; // Assuming Type and SymbolKind are needed

#[test]
fn test_new_node() {
    let node = AudioShaderNode::new();
    assert_eq!(node.get_shader(), "");
    assert!(node.program.is_none());
    assert!(node.input.is_empty());
    assert!(node.output.is_empty());
}

#[test]
fn test_set_and_get_shader_simple() {
    let mut node = AudioShaderNode::new();
    let shader_code = "input number in1\noutput number out1\nout1 = in1 * 2.0";
    assert!(node.set_shader(shader_code.to_string()).is_ok());
    assert_eq!(node.get_shader(), shader_code);
    assert!(node.program.is_some());
}

#[test]
fn test_set_shader_populates_io_tables() {
    let mut node = AudioShaderNode::new();
    let shader_code =
        "input number input_value\ninput number input_buf\noutput number output_value";
    let result = node.set_shader(shader_code.to_string());
    assert!(result.is_ok(), "Expected Ok, got {:?}", result.err());

    assert!(node.input.contains_key("input_value"));
    let input_val_info = node.input.get("input_value").unwrap();
    assert_eq!(input_val_info.kind, SymbolKind::Input);

    assert!(node.input.contains_key("input_buf"));
    let input_buf_info = node.input.get("input_buf").unwrap();
    assert_eq!(input_buf_info.kind, SymbolKind::Input);

    assert!(node.output.contains_key("output_value"));
    let output_val_info = node.output.get("output_value").unwrap();
    assert_eq!(output_val_info.kind, SymbolKind::Output);
}

#[test]
fn test_set_shader_syntax_error() {
    let mut node = AudioShaderNode::new();
    let shader_code = "input number val output number res res = val"; // No line breaks
    let result = node.set_shader(shader_code.to_string());
    assert!(result.is_err());
    assert!(node.program.is_none());
    // Check if input/output tables are cleared or remain empty
    assert!(node.input.is_empty());
    assert!(node.output.is_empty());
}

#[test]
fn test_set_shader_semantic_error() {
    // Using an undeclared variable
    let shader_code = "input number in1\noutput number out1\nout1 = undeclared_var * 2.0";

    // Compile the shader code into a program.
    let lexer = Lexer::new(shader_code.to_string());
    let tokens = lexer.tokenize();
    let parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    // Check for errors in the program.
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    println!("{:?}", program);
    assert!(result.is_err());
}

#[test]
fn test_process_simple_float_shader() {
    let mut node = AudioShaderNode::new();
    let shader_code = "input number a\noutput number b\nb = a * 2.5";
    assert!(node.set_shader(shader_code.to_string()).is_ok());

    node.set_input("a", Value::Float(10.0));
    let process_result = node.process(48000, 24000.0, 1, 0, 1, 0);
    assert!(
        process_result.is_ok(),
        "Process failed: {:?}",
        process_result.err()
    );

    match node.get_output("b") {
        Some(Value::Float(val)) => assert_eq!(val, 25.0),
        other => panic!("Expected Some(Value::Float(25.0)), got {:?}", other),
    }
}

#[test]
fn test_multiple_args() {
    let mut node = AudioShaderNode::new();
    let shader_code = "input number a\ninput number b\noutput number c\nc = max(a, b)";

    let set_shader_result = node.set_shader(shader_code.to_string());
    assert!(
        set_shader_result.is_ok(),
        "Failed to set shader: {:?}",
        set_shader_result.err()
    );

    node.set_input("a", Value::Float(3.0));
    node.set_input("b", Value::Float(4.0));

    let process_result = node.process(48000, 24000.0, 1, 0, 1, 0);
    assert!(
        process_result.is_ok(),
        "Process failed: {:?}",
        process_result.err()
    );

    match node.get_output("c") {
        Some(Value::Float(val)) => assert_eq!(val, 4.0),
        other => panic!("Expected Some(Value::Float(4.0)), got {:?}", other),
    }
}
