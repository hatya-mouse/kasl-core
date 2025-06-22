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
    AssignmentStatement, Expression, InputDeclarationStatement, Operator,
    OutputDeclarationStatement, Program, Statement, TokenType, Type, VariableDeclarationStatement,
};

pub struct Parser {
    tokens: Vec<TokenType>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<Program, String> {
        let lines: Vec<Vec<TokenType>> = self
            .tokens
            .split(|token| *token == TokenType::EndOfLine)
            .map(|line| line.to_vec())
            .collect();
        let mut program = Program {
            statements: Vec::new(),
        };

        for line in lines {
            self.parse_line(line, &mut program)?;
        }

        Ok(program)
    }

    fn parse_line(&self, line: Vec<TokenType>, program: &mut Program) -> Result<(), String> {
        let mut token_iter = line.iter().peekable();

        while let Some(token) = token_iter.next() {
            match token {
                TokenType::Input => {
                    let data_type = match token_iter.next() {
                        Some(TokenType::Float) => Type::Float,
                        Some(TokenType::Buffer) => Type::Buffer,
                        _ => {
                            return Err("Expected type after 'input'".into());
                        }
                    };

                    let name = match token_iter.next() {
                        Some(TokenType::Identifier(name)) => name.clone(),
                        _ => {
                            return Err("Expected identifier after type".into());
                        }
                    };

                    let initial_value = match token_iter.next() {
                        Some(TokenType::Assign) => Some(self.parse_expression(&mut token_iter)?),
                        _ => None,
                    };

                    program.statements.push(Statement::InputDeclaration(
                        InputDeclarationStatement {
                            name,
                            data_type,
                            initial_value,
                            range: None,
                        },
                    ));
                }

                TokenType::InputRange(start, end) => {
                    let data_type = match token_iter.next() {
                        Some(TokenType::Float) => Type::Float,
                        Some(TokenType::Buffer) => Type::Buffer,
                        _ => {
                            return Err("Expected type after 'input'".into());
                        }
                    };

                    let name = match token_iter.next() {
                        Some(TokenType::Identifier(name)) => name.clone(),
                        _ => {
                            return Err("Expected identifier after type".into());
                        }
                    };

                    let initial_value = match token_iter.next() {
                        Some(TokenType::Assign) => Some(self.parse_expression(&mut token_iter)?),
                        _ => None,
                    };

                    program.statements.push(Statement::InputDeclaration(
                        InputDeclarationStatement {
                            name,
                            data_type,
                            initial_value,
                            range: Some((*start, *end)),
                        },
                    ));
                }

                TokenType::Output => {
                    let data_type = match token_iter.next() {
                        Some(TokenType::Float) => Type::Float,
                        Some(TokenType::Buffer) => Type::Buffer,
                        _ => {
                            return Err("Expected type after 'output'".into());
                        }
                    };

                    let name = match token_iter.next() {
                        Some(TokenType::Identifier(name)) => name.clone(),
                        _ => {
                            return Err("Expected identifier after type".into());
                        }
                    };

                    program.statements.push(Statement::OutputDeclaration(
                        OutputDeclarationStatement { name, data_type },
                    ));
                }

                TokenType::Float | TokenType::Buffer => {
                    let name = match token_iter.next() {
                        Some(TokenType::Identifier(name)) => name.clone(),
                        _ => {
                            return Err("Expected identifier after type name".into());
                        }
                    };

                    let data_type = match token {
                        TokenType::Float => Type::Float,
                        TokenType::Buffer => Type::Buffer,
                        _ => unreachable!(),
                    };

                    let initial_value = match token_iter.next() {
                        Some(TokenType::Assign) => self.parse_expression(&mut token_iter),
                        _ => Err("Expected '=' after identifier".into()),
                    }?;

                    program.statements.push(Statement::VariableDeclaration(
                        VariableDeclarationStatement {
                            name,
                            data_type,
                            initial_value,
                        },
                    ));
                }

                TokenType::Identifier(name) => {
                    let target_name = name.clone();

                    match token_iter.next() {
                        Some(TokenType::Assign) => {
                            // Assignment statement
                            let value = self.parse_expression(&mut token_iter)?;

                            program
                                .statements
                                .push(Statement::Assignment(AssignmentStatement {
                                    target_name,
                                    value,
                                }));
                            continue; // Skip to the next token
                        }

                        _ => return Err(format!("Expected '=' after identifier '{}'", name)),
                    }
                }

                TokenType::EndOfFile => {
                    // End of file, we can stop parsing
                    break;
                }

                _ => {}
            }
        }

        Ok(())
    }

    /// Parses an expression recursively from the token iterator.
    fn parse_expression(
        &self,
        token_iter: &mut std::iter::Peekable<std::slice::Iter<TokenType>>,
    ) -> Result<Expression, String> {
        let mut left = match token_iter.next() {
            Some(TokenType::FloatLiteral(value)) => Expression::Literal(*value),
            Some(TokenType::Identifier(name)) => match token_iter.peek() {
                Some(TokenType::LParen) => {
                    token_iter.next();
                    let mut args = Vec::new();
                    while let Some(arg) = self.parse_expression(token_iter).ok() {
                        args.push(arg);
                        if token_iter.peek() == Some(&&TokenType::Comma) {
                            token_iter.next();
                        } else {
                            break;
                        }
                    }
                    Expression::FunctionCall {
                        name: name.clone(),
                        arguments: args,
                    }
                }
                _ => Expression::Identifier(name.clone()),
            },
            Some(TokenType::LParen) => {
                let expr = self.parse_expression(token_iter)?;
                match token_iter.next() {
                    Some(TokenType::RParen) => expr,
                    _ => return Err("Expected ')'".into()),
                }
            }
            _ => return Err("Expected a literal, identifier, or '('".into()),
        };

        while let Some(op) = token_iter.peek() {
            match op {
                TokenType::Plus | TokenType::Minus | TokenType::Multiply | TokenType::Divide => {
                    let operator = match op {
                        TokenType::Plus => Operator::Add,
                        TokenType::Minus => Operator::Subtract,
                        TokenType::Multiply => Operator::Multiply,
                        TokenType::Divide => Operator::Divide,
                        _ => unreachable!(),
                    };
                    token_iter.next(); // consume the operator

                    let right = self.parse_expression(token_iter)?;
                    left = Expression::BinaryOp {
                        op: operator,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }
}
