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

use knodiq_engine::Type;

use crate::{
    AssignmentStatement, Expression, InputDeclarationStatement, Lexer, Operator,
    OutputDeclarationStatement, Program, Statement, TokenType, VariableDeclarationStatement,
    ast::ForLoopStatement, token_type::Token,
};

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, program: &str) -> Result<Program, String> {
        let tokens = Lexer::new(program.to_string()).tokenize();

        let lines: Vec<Vec<Token>> = tokens
            .split(|token| token.token_type == TokenType::EndOfLine)
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
        lines: &mut Peekable<Iter<Vec<Token>>>,
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
                match &token.token_type {
                    TokenType::Input => {
                        let value_type = self.parse_type(&mut token_iter)?;

                        let name = match token_iter.next().map(|t| &t.token_type) {
                            Some(TokenType::Identifier(name)) => name.clone(),
                            _ => {
                                return Err("Expected identifier after type".into());
                            }
                        };

                        statements.push(Statement::InputDeclaration(InputDeclarationStatement {
                            name,
                            input_attrs: Vec::new(),
                            value_type,
                            line: token.line,
                        }));
                    }

                    TokenType::Output => {
                        let value_type = self.parse_type(&mut token_iter)?;

                        let name = match token_iter.next().map(|t| &t.token_type) {
                            Some(TokenType::Identifier(name)) => name.clone(),
                            _ => {
                                return Err("Expected identifier after type".into());
                            }
                        };

                        statements.push(Statement::OutputDeclaration(OutputDeclarationStatement {
                            name,
                            value_type,
                            line: token.line,
                        }));
                    }

                    TokenType::Var => {
                        let value_type = self.parse_type(&mut token_iter)?;

                        let name = match token_iter.next().map(|t| &t.token_type) {
                            Some(TokenType::Identifier(name)) => name.clone(),
                            _ => {
                                return Err("Expected identifier after type name".into());
                            }
                        };

                        let initial_value = match token_iter.next().map(|t| &t.token_type) {
                            Some(TokenType::Assign) => self.parse_expression(&mut token_iter),
                            _ => Err(format!("Expected '=' after identifier {}", name)),
                        }?;

                        statements.push(Statement::VariableDeclaration(
                            VariableDeclarationStatement {
                                name,
                                initial_value,
                                value_type,
                                line: token.line,
                            },
                        ));
                    }

                    TokenType::Identifier(name) => {
                        let target_name = name.clone();

                        match token_iter.next().map(|t| &t.token_type) {
                            Some(TokenType::Assign) => {
                                // Assignment statement
                                let value = self.parse_expression(&mut token_iter)?;

                                statements.push(Statement::Assignment(AssignmentStatement {
                                    target_name,
                                    value,
                                    line: token.line,
                                }));
                                continue; // Skip to the next token
                            }

                            _ => return Err(format!("Expected '=' after identifier '{}'", name)),
                        }
                    }

                    TokenType::For => {
                        let variable_name = match token_iter.next().map(|t| &t.token_type) {
                            Some(TokenType::Identifier(name)) => name.clone(),
                            _ => return Err("Expected identifier after 'for'".into()),
                        };

                        if token_iter.next().map(|t| &t.token_type) != Some(&TokenType::In) {
                            return Err("Expected 'in' after for variable".into());
                        }

                        let iterable = self.parse_expression(&mut token_iter)?;

                        if token_iter.next().map(|t| &t.token_type) != Some(&TokenType::LBrace) {
                            return Err("Expected '{' after iterable in for loop".into());
                        }

                        let body = self.parse_block(depth, lines)?;

                        statements.push(Statement::ForLoop(ForLoopStatement {
                            variable_name,
                            iterable,
                            body,
                            line: token.line,
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
        token_iter: &mut Peekable<Iter<Token>>,
    ) -> Result<Expression, String> {
        let mut left = match token_iter.next().map(|t| &t.token_type) {
            Some(TokenType::FloatLiteral(value)) => Expression::Literal(*value),
            Some(TokenType::Identifier(name)) => match token_iter.peek().map(|t| &t.token_type) {
                Some(TokenType::LParen) => {
                    token_iter.next();
                    let mut args = Vec::new();
                    while token_iter.peek().map(|t| &t.token_type) != Some(&&TokenType::RParen) {
                        let arg = self.parse_expression(token_iter)?;
                        args.push(arg);
                        if token_iter.peek().map(|t| &t.token_type) == Some(&&TokenType::Comma) {
                            token_iter.next();
                        } else if token_iter.peek().is_none() {
                            return Err("Expected ')'".into());
                        }
                    }
                    token_iter.next(); // consume ')'
                    Expression::FunctionCall {
                        name: name.clone(),
                        arguments: args,
                    }
                }
                _ => Expression::Identifier(name.clone()),
            },
            Some(TokenType::LParen) => {
                let expr = self.parse_expression(token_iter)?;
                match token_iter.next().map(|t| &t.token_type) {
                    Some(TokenType::RParen) => expr,
                    _ => return Err("Expected ')'".into()),
                }
            }
            _ => return Err("Expected a literal, identifier, or '('".into()),
        };

        while let Some(op) = token_iter.peek().map(|t| &t.token_type) {
            match op {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Multiply
                | TokenType::Divide
                | TokenType::Modulo => {
                    let operator = match op {
                        TokenType::Plus => Operator::Add,
                        TokenType::Minus => Operator::Subtract,
                        TokenType::Multiply => Operator::Multiply,
                        TokenType::Divide => Operator::Divide,
                        TokenType::Modulo => Operator::Modulo,
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

    fn parse_type(&self, token_iter: &mut Peekable<Iter<Token>>) -> Result<Type, String> {
        match token_iter.next().map(|t| &t.token_type) {
            Some(TokenType::Identifier(name)) => match name.as_str() {
                "float" => Ok(Type::Float),
                "int" => Ok(Type::Int),
                _ => Err(format!("Unknown type '{}'", name)),
            },
            Some(TokenType::LBrace) => {
                let token: Vec<Token> = token_iter
                    .take_while(|t| t.token_type != TokenType::RBrace)
                    .cloned()
                    .collect();
                Ok(Type::Array(Box::new(
                    self.parse_type(&mut token.iter().peekable())?,
                )))
            }
            _ => Err("Expected type identifier".into()),
        }
    }
}
