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

use knodiq_audio_shader::{Compiler, Expression, Parser, Statement};
use knodiq_engine::{Type, Value};

#[test]
fn test_parsing() {
    let code = "var x = 1.0\noutput float y\ny = pow(x, 2)";

    let parser = Parser::new();
    let program = parser.parse(&code);

    assert!(program.is_ok());
    let program = program.unwrap();
    assert!(!program.statements.is_empty());

    let var_stmt = match &program.statements[0] {
        Statement::VariableDeclaration(stmt) => stmt,
        _ => panic!("Expected VariableDeclarationStatement"),
    };
    assert_eq!(var_stmt.name, "x");

    let output_stmt = match &program.statements[1] {
        Statement::OutputDeclaration(stmt) => stmt,
        _ => panic!("Expected OutputDeclarationStatement"),
    };
    assert_eq!(output_stmt.name, "y");

    let assignment_stmt = match &program.statements[2] {
        Statement::Assignment(stmt) => stmt,
        _ => panic!("Expected AssignmentStatement"),
    };
    assert_eq!(assignment_stmt.target_name, "y");
    assert_eq!(
        assignment_stmt.value,
        Expression::FunctionCall {
            name: "pow".to_string(),
            arguments: vec![
                Expression::Identifier("x".to_string()),
                Expression::FloatLiteral(2.0)
            ]
        }
    );
}

// #[test]
// fn test_interpreter_basic() {
//     let code = "input float in_buffer
//                     output float out_buffer
//                     output float powered
//                     var gain = 1.0
//                     var result = 0.0

//                     result = in_buffer * gain
//                     out_buffer = result + 1.25

//                     powered = pow(in_buffer, 2.0)";

//     let parser = Parser::new();
//     let program: Program = parser.parse(&code).unwrap();

//     let mut analyzer = SemanticAnalyzer::new();
//     analyzer.analyze(&program).unwrap();

//     let mut interpreter = Interpreter::new(program, 48000, 24000.0, 2, 0, 2);

//     let mut input_table = analyzer.input_table.clone();
//     input_table.get_mut("in_buffer").unwrap().value =
//         Some(Value::from_buffer(vec![vec![2.0, 3.0]; 2]));

//     let output_table = interpreter.execute(input_table).unwrap();

//     assert_eq!(
//         output_table.get("out_buffer").unwrap().value,
//         Some(Value::from_buffer(vec![vec![3.25, 4.25]; 2]))
//     );
//     assert_eq!(
//         output_table.get("powered").unwrap().value,
//         Some(Value::from_buffer(vec![vec![4.0, 9.0]; 2]))
//     );
// }

// #[test]
// fn test_interpreter_advanced() {
//     let code = "input float in_buffer
//                     output float out_buffer

//                     out_buffer = in_buffer * sin(time() * pi() * 440)";

//     let parser = Parser::new();
//     let program: Program = parser.parse(&code).unwrap();

//     let mut analyzer = SemanticAnalyzer::new();
//     analyzer.analyze(&program).unwrap();

//     let mut interpreter = Interpreter::new(program, 48000, 24000.0, 2, 0, 2);

//     let mut input_table = analyzer.input_table.clone();
//     input_table.get_mut("in_buffer").unwrap().value =
//         Some(Value::from_buffer(vec![vec![2.0, 3.0]; 2]));

//     assert!(interpreter.execute(input_table).is_ok());
// }

#[test]
fn test_compiler() {
    let code = "input float in_buffer
                    output float out_buffer
                    output float powered
                    var gain = 1.5
                    var result = 0.0

                    result = in_buffer * gain
                    out_buffer = result + 1.25

                    powered = pow(in_buffer, 2.0)";

    let mut inputs = Vec::new();
    inputs.push(Value::Array(vec![Value::Float(2.0), Value::Float(3.0)]));

    let mut compiler = Compiler::new().unwrap();
    let mut exec = compiler.compile(&code).unwrap();
    let result = exec
        .run(
            inputs,
            vec![
                Type::Array(Box::new(Type::Float)),
                Type::Array(Box::new(Type::Float)),
            ],
        )
        .unwrap();
    println!("Compiler run result: {:?}", result);
}

#[test]
#[ignore]
fn test_compiler_arr() {
    let code = "input [float] in_buffer
                    output [float] out_buffer
                    output [float] powered
                    var gain = 1.5
                    var result = 0.0

                    result = in_buffer * gain
                    out_buffer = result + 1.25

                    powered = in_buffer * in_buffer";

    let mut inputs = Vec::new();
    inputs.push(Value::Array(vec![Value::Float(2.0), Value::Float(3.0)]));

    let mut compiler = Compiler::new().unwrap();
    let mut exec = compiler.compile(&code).unwrap();
    println!("Compiled code");
    let result = exec
        .run(
            inputs,
            vec![
                Type::Array(Box::new(Type::Float)),
                Type::Array(Box::new(Type::Float)),
            ],
        )
        .unwrap();
    println!("Compiler run result: {:?}", result);
}
