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
    Expression, Interpreter, Lexer, Parser, Program, SemanticAnalyzer, Statement, TokenType, Type,
    Value,
};

#[test]
fn test_basic_shader_tokenize() {
    let code = "input float in_buffer = 0.0
                    input float gain = 1.0
                    output float out_buffer
                    out_buffer = in_buffer * gain";
    let lexer = Lexer::new(code.to_string());
    let tokens = lexer.tokenize();

    assert!(!tokens.is_empty());
    assert_eq!(tokens[0].token_type, TokenType::Input);
    assert_eq!(
        tokens[2].token_type,
        TokenType::Identifier("in_buffer".into())
    );
    assert_eq!(tokens[3].token_type, TokenType::Assign);
    assert_eq!(tokens[4].token_type, TokenType::FloatLiteral(0.0));

    assert_eq!(tokens[10].token_type, TokenType::FloatLiteral(1.0));
}

#[test]
fn test_parsing() {
    let code = "input number x = 1.0
                    output number y

                    y = pow(x, 2)";
    let lexer = Lexer::new(code.to_string());
    let tokens = lexer.tokenize();

    let parser = Parser::new(tokens);
    let program = parser.parse();

    assert!(program.is_ok());
    let program = program.unwrap();
    assert!(!program.statements.is_empty());

    let input_stmt = match &program.statements[0] {
        Statement::InputDeclaration(stmt) => stmt,
        _ => panic!("Expected InputDeclarationStatement"),
    };
    assert_eq!(input_stmt.name, "x");
    assert_eq!(input_stmt.data_type, Type::Float);
    assert_eq!(input_stmt.initial_value, Some(Expression::Literal(1.0)));

    let output_stmt = match &program.statements[1] {
        Statement::OutputDeclaration(stmt) => stmt,
        _ => panic!("Expected OutputDeclarationStatement"),
    };
    assert_eq!(output_stmt.name, "y");
    assert_eq!(output_stmt.data_type, Type::Float);

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
                Expression::Literal(2.0)
            ]
        }
    );
}

#[test]
fn test_interpreter() {
    let code = "input number in_buffer
                    input number gain = 1.0
                    var result = 0.0
                    output number out_buffer
                    output number powered

                    result = in_buffer * gain
                    out_buffer = result + 1.25
                    
                    powered = pow(in_buffer, 2.0)";
    let lexer = Lexer::new(code.to_string());
    let tokens = lexer.tokenize();

    let parser = Parser::new(tokens);
    let program: Program = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program).unwrap();

    let mut interpreter = Interpreter::new(program, 48000, 24000.0, 2, 0, 2);

    let mut input_table = analyzer.input_table.clone();
    input_table.get_mut("in_buffer").unwrap().value =
        Some(Value::from_buffer(vec![vec![2.0, 3.0]; 2]));
    input_table.get_mut("gain").unwrap().value = Some(Value::Float(1.5));

    let output_table = interpreter.execute(input_table).unwrap();

    assert_eq!(
        output_table.get("out_buffer").unwrap().value,
        Some(Value::from_buffer(vec![vec![4.25, 5.75]; 2]))
    );
    assert_eq!(
        output_table.get("powered").unwrap().value,
        Some(Value::from_buffer(vec![vec![4.0, 9.0]; 2]))
    );
}
