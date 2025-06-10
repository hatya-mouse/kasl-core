use knodiq_audio_shader::{AudioShaderNode, Node, SymbolKind, Type, Value}; // Assuming Type and SymbolKind are needed
use std::collections::HashSet;

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
    let shader_code = "input float in1;\noutput float out1;\nout1 = in1 * 2.0;";
    assert!(node.set_shader(shader_code.to_string()).is_ok());
    assert_eq!(node.get_shader(), shader_code);
    assert!(node.program.is_some());
}

#[test]
fn test_set_shader_populates_io_tables() {
    let mut node = AudioShaderNode::new();
    let shader_code =
        "input float input_value;\ninput buffer input_buf;\noutput float output_value;";
    let result = node.set_shader(shader_code.to_string());
    assert!(result.is_ok(), "Expected Ok, got {:?}", result.err());

    assert!(node.input.contains_key("input_value"));
    let input_val_info = node.input.get("input_value").unwrap();
    assert_eq!(input_val_info.data_type, Type::Float);
    assert_eq!(input_val_info.kind, SymbolKind::Input);

    assert!(node.input.contains_key("input_buf"));
    let input_buf_info = node.input.get("input_buf").unwrap();
    assert_eq!(input_buf_info.data_type, Type::Buffer);
    assert_eq!(input_buf_info.kind, SymbolKind::Input);

    assert!(node.output.contains_key("output_value"));
    let output_val_info = node.output.get("output_value").unwrap();
    assert_eq!(output_val_info.data_type, Type::Float);
    assert_eq!(output_val_info.kind, SymbolKind::Output);
}

#[test]
fn test_set_shader_syntax_error() {
    let mut node = AudioShaderNode::new();
    let shader_code = "input float val output float res; res = val;"; // Missing semicolon
    let result = node.set_shader(shader_code.to_string());
    assert!(result.is_err());
    assert!(node.program.is_none());
    // Check if input/output tables are cleared or remain empty
    assert!(node.input.is_empty());
    assert!(node.output.is_empty());
}

#[test]
fn test_set_shader_semantic_error() {
    let mut node = AudioShaderNode::new();
    // Using an undeclared variable
    let shader_code = "input float in1;\noutput float out1;\nout1 = undeclared_var * 2.0;";
    let result = node.set_shader(shader_code.to_string());
    assert!(result.is_err());
    assert!(node.program.is_none());
    assert!(node.input.is_empty());
    assert!(node.output.is_empty());
}

#[test]
fn test_process_simple_float_shader() {
    let mut node = AudioShaderNode::new();
    let shader_code = "input float a;\noutput float b;\nb = a * 2.5;";
    assert!(node.set_shader(shader_code.to_string()).is_ok());

    node.set_input("a", Value::Float(10.0));
    let process_result = node.process(48000, 1, 0, 1);
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
fn test_process_simple_buffer_shader() {
    let mut node = AudioShaderNode::new();
    let shader_code = "input buffer input_signal;\noutput buffer output_signal;\noutput_signal = input_signal * 0.5;";
    assert!(node.set_shader(shader_code.to_string()).is_ok());

    let input_buffer_data = vec![vec![10.0, 20.0, 30.0]]; // 1 channel, 3 samples
    node.set_input("input_signal", Value::Buffer(input_buffer_data.clone()));

    // Process 1 channel, 3 samples
    let process_result = node.process(48000, 1, 0, 3);
    assert!(
        process_result.is_ok(),
        "Process failed: {:?}",
        process_result.err()
    );

    let expected_output_data = vec![vec![5.0, 10.0, 15.0]];
    match node.get_output("output_signal") {
        Some(Value::Buffer(val)) => assert_eq!(val, expected_output_data),
        other => panic!("Expected Some(Value::Buffer(...)), got {:?}", other),
    }
}

#[test]
fn test_get_set_input() {
    let mut node = AudioShaderNode::new();
    let shader_code = "input float my_in;\noutput float my_out;\nmy_out = my_in;";
    let result = node.set_shader(shader_code.to_string());
    assert!(result.is_ok(), "Failed to set shader: {:?}", result.err());

    node.set_input("my_in", Value::Float(123.0));
    assert_eq!(node.get_input("my_in"), Some(Value::Float(123.0)));

    assert_eq!(node.get_input("non_existent_input"), None);

    // Try setting an input not declared in the shader (should not be added)
    node.set_input("another_key_not_in_shader", Value::Float(456.0));
    assert_eq!(node.get_input("another_key_not_in_shader"), None);
}

#[test]
fn test_get_output_before_and_after_process() {
    let mut node = AudioShaderNode::new();
    let shader_code = "input float val_in;\noutput float val_out;\nval_out = val_in + 1.0;";
    assert!(node.set_shader(shader_code.to_string()).is_ok());

    // Before process, output might be None or default depending on interpreter initialization
    // The current `AudioShaderNode` initializes `output` from analyzer, which has `None` values.
    match node.output.get("val_out") {
        Some(symbol_info) => assert!(symbol_info.value.is_none()),
        None => panic!("Output key 'val_out' should exist after set_shader"),
    }
    assert_eq!(node.get_output("val_out"), None); // `get_output` unwraps SymbolInfo's value

    node.set_input("val_in", Value::Float(5.0));
    assert!(node.process(48000, 1, 0, 1).is_ok());

    assert_eq!(node.get_output("val_out"), Some(Value::Float(6.0)));
    assert_eq!(node.get_output("non_existent_output"), None);
}

#[test]
fn test_get_input_output_lists() {
    let mut node = AudioShaderNode::new();
    let shader_code = "input float in_f;\ninput buffer in_b;\noutput float out_f;\noutput buffer out_b;\nout_f=in_f; out_b=in_b;";
    assert!(node.set_shader(shader_code.to_string()).is_ok());

    let expected_inputs: HashSet<String> = ["in_f", "in_b"].iter().map(|s| s.to_string()).collect();
    let actual_inputs: HashSet<String> = node.get_input_list().into_iter().collect();
    assert_eq!(actual_inputs, expected_inputs);

    let expected_outputs: HashSet<String> =
        ["out_f", "out_b"].iter().map(|s| s.to_string()).collect();
    let actual_outputs: HashSet<String> = node.get_output_list().into_iter().collect();
    assert_eq!(actual_outputs, expected_outputs);
}

#[test]
fn test_clone_node() {
    let mut original_node = AudioShaderNode::new();
    let shader_code = "input float x;\noutput float y;\ny = x + 10.0;";
    assert!(original_node.set_shader(shader_code.to_string()).is_ok());
    original_node.set_input("x", Value::Float(5.0));
    assert!(original_node.process(48000, 1, 0, 1).is_ok());

    let cloned_node = original_node.clone();

    assert_eq!(cloned_node.get_shader(), shader_code);
    assert!(cloned_node.program.is_some());
    assert_eq!(cloned_node.get_input("x"), Some(Value::Float(5.0)));
    assert_eq!(cloned_node.get_output("y"), Some(Value::Float(15.0)));

    // Modify original and check if clone is independent
    original_node.set_input("x", Value::Float(20.0));
    assert!(original_node.process(48000, 1, 0, 1).is_ok());
    assert_eq!(original_node.get_output("y"), Some(Value::Float(30.0)));

    // Cloned node should remain unchanged
    assert_eq!(cloned_node.get_input("x"), Some(Value::Float(5.0))); // Input was cloned at that state
    assert_eq!(cloned_node.get_output("y"), Some(Value::Float(15.0))); // Output was cloned at that state
}

#[test]
fn test_process_with_no_program_should_panic_or_err() {
    // This test depends on how `process` handles `self.program` being `None`.
    // Current implementation uses `unwrap()`, so it will panic.
    // A more robust implementation might return `Err`.
    let mut node = AudioShaderNode::new();
    // Do not set shader, so program is None

    // We expect a panic here. To test for panics:
    let result = std::panic::catch_unwind(move || node.process(48000, 1, 0, 1));
    assert!(
        result.is_err(),
        "Process should panic if program is not set"
    );
}
