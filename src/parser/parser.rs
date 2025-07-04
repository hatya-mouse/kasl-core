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

use std::{iter::Peekable, slice::Iter};

use crate::{
    AssignmentStatement, Expression, InputDeclarationStatement, Operator,
    OutputDeclarationStatement, Program, Statement, TokenType, Type, VariableDeclarationStatement,
    ast::ForLoopStatement,
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

        program.statements = self.parse_block(0, &mut lines.iter().peekable())?;

        Ok(program)
    }

    fn parse_block(
        &self,
        depth: usize,
        lines: &mut Peekable<Iter<Vec<TokenType>>>,
    ) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        let mut has_ended = false;

        while !has_ended {
            let line = match lines.next() {
                Some(line) => line,
                None => {
                    has_ended = true;
                    continue; // No more lines to process
                }
            };
            let mut token_iter = line.iter().peekable();

            while let Some(token) = token_iter.next() {
                match token {
                    TokenType::Input => {
                        let mut range: Option<(f32, f32)> = None;

                        // Parse value range
                        if let Some(TokenType::LParen) = token_iter.peek() {
                            token_iter.next(); // consume '('
                            let start = match token_iter.next() {
                                Some(TokenType::FloatLiteral(value)) => value,
                                _ => {
                                    return Err(
                                        "Expected float literal after '('. Range start missing."
                                            .into(),
                                    );
                                }
                            };

                            let end = match token_iter.next() {
                                Some(TokenType::FloatLiteral(value)) => value,
                                _ => {
                                    return Err(
                                    "Expected float literal after range start. Range end missing."
                                        .into(),
                                );
                                }
                            };

                            if token_iter.next() != Some(&TokenType::RParen) {
                                return Err("Expected ')' to close range".into());
                            }

                            range = Some((*start, *end));
                        }

                        let data_type = self.parse_type(&mut token_iter)?;

                        let name = match token_iter.next() {
                            Some(TokenType::Identifier(name)) => name.clone(),
                            _ => {
                                return Err("Expected identifier after type".into());
                            }
                        };

                        let initial_value = match token_iter.next() {
                            Some(TokenType::Assign) => {
                                Some(self.parse_expression(&mut token_iter)?)
                            }
                            _ => None,
                        };

                        statements.push(Statement::InputDeclaration(InputDeclarationStatement {
                            name,
                            data_type,
                            initial_value,
                            range,
                        }));
                    }

                    TokenType::Output => {
                        let data_type = self.parse_type(&mut token_iter)?;

                        let name = match token_iter.next() {
                            Some(TokenType::Identifier(name)) => name.clone(),
                            _ => {
                                return Err("Expected identifier after type".into());
                            }
                        };

                        statements.push(Statement::OutputDeclaration(OutputDeclarationStatement {
                            name,
                            data_type,
                        }));
                    }

                    TokenType::Var => {
                        let name = match token_iter.next() {
                            Some(TokenType::Identifier(name)) => name.clone(),
                            _ => {
                                return Err("Expected identifier after type name".into());
                            }
                        };

                        let initial_value = match token_iter.next() {
                            Some(TokenType::Assign) => self.parse_expression(&mut token_iter),
                            _ => Err("Expected '=' after identifier".into()),
                        }?;

                        statements.push(Statement::VariableDeclaration(
                            VariableDeclarationStatement {
                                name,
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

                                statements.push(Statement::Assignment(AssignmentStatement {
                                    target_name,
                                    value,
                                }));
                                continue; // Skip to the next token
                            }

                            _ => return Err(format!("Expected '=' after identifier '{}'", name)),
                        }
                    }

                    TokenType::For => {
                        let variable_name = match token_iter.next() {
                            Some(TokenType::Identifier(name)) => name.clone(),
                            _ => return Err("Expected identifier after 'for'".into()),
                        };

                        if token_iter.next() != Some(&TokenType::In) {
                            return Err("Expected 'in' after for variable".into());
                        }

                        let iterable = self.parse_expression(&mut token_iter)?;

                        if token_iter.next() != Some(&TokenType::LBrace) {
                            return Err("Expected '{' after iterable in for loop".into());
                        }

                        let body = self.parse_block(depth, lines)?;

                        statements.push(Statement::ForLoop(ForLoopStatement {
                            variable_name,
                            iterable,
                            body,
                        }));
                    }

                    TokenType::RBrace => {
                        // End of a block, we can return the current depth
                        if depth == 0 {
                            return Err("Unexpected '}' without matching '{'".into());
                        }
                        has_ended = true;
                    }

                    TokenType::EndOfFile => {
                        // End of file, we can stop parsing
                        has_ended = true;
                    }

                    _ => {}
                }
            }
        }

        Ok(statements)
    }

    /// Parses an expression recursively from the token iterator.
    fn parse_expression(
        &self,
        token_iter: &mut Peekable<Iter<TokenType>>,
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

    /// Parses a type from the token iterator.
    fn parse_type(&self, token_iter: &mut Peekable<Iter<TokenType>>) -> Result<Type, String> {
        let base_type = match token_iter.next() {
            Some(TokenType::Number) => Type::Float,
            _ => {
                return Err("Expected type".into());
            }
        };

        // Check for array brackets after base type
        let mut current_type = base_type;
        while token_iter.peek() == Some(&&TokenType::LBracket) {
            token_iter.next(); // consume '['

            match token_iter.next() {
                Some(TokenType::RBracket) => {
                    current_type = Type::Array(Box::new(current_type));
                }
                _ => {
                    return Err("Expected ']' after '[' in type literal".into());
                }
            }
        }

        Ok(current_type)
    }
}
