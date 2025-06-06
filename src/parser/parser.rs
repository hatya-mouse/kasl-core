use crate::{
    AssignmentStatement, Expression, InputDeclarationStatement, Operator,
    OutputDeclarationStatement, Program, Statement, TokenType, Type, VariableDeclarationStatement,
};

pub struct Parser {
    tokens: Vec<TokenType>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        println!("{:?}", tokens);
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<Program, String> {
        let mut token_iter = self.tokens.iter().peekable();
        let mut tree = Program {
            statements: Vec::new(),
        };

        while let Some(token) = token_iter.next() {
            match token {
                TokenType::Input => {
                    let data_type = match token_iter.next() {
                        Some(TokenType::Buffer) => Type::Buffer,
                        Some(TokenType::Float) => Type::Float,
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
                        Some(TokenType::Assign) => match token_iter.next() {
                            Some(TokenType::FloatLiteral(value)) => Expression::Literal(*value),
                            Some(TokenType::Identifier(name)) => {
                                Expression::Identifier(name.clone())
                            }
                            _ => return Err("Expected initial value after '='".into()),
                        },
                        _ => return Err("Expected initial value after identifier".into()),
                    };

                    tree.statements
                        .push(Statement::InputDeclaration(InputDeclarationStatement {
                            name,
                            data_type,
                            initial_value,
                            range: None,
                        }));
                }

                TokenType::InputRange(start, end) => {
                    let data_type = match token_iter.next() {
                        Some(TokenType::Buffer) => Type::Buffer,
                        Some(TokenType::Float) => Type::Float,
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
                        Some(TokenType::Assign) => match token_iter.next() {
                            Some(TokenType::FloatLiteral(value)) => Expression::Literal(*value),
                            Some(TokenType::Identifier(name)) => {
                                Expression::Identifier(name.clone())
                            }
                            _ => return Err("Expected initial value after '='".into()),
                        },
                        _ => return Err("Expected initial value after identifier".into()),
                    };

                    tree.statements
                        .push(Statement::InputDeclaration(InputDeclarationStatement {
                            name,
                            data_type,
                            initial_value,
                            range: Some((*start, *end)),
                        }));
                }

                TokenType::Output => {
                    let data_type = match token_iter.next() {
                        Some(TokenType::Buffer) => Type::Buffer,
                        Some(TokenType::Float) => Type::Float,
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

                    tree.statements.push(Statement::OutputDeclaration(
                        OutputDeclarationStatement { name, data_type },
                    ));
                }

                TokenType::Buffer | TokenType::Float => {
                    let name = match token_iter.next() {
                        Some(TokenType::Identifier(name)) => name.clone(),
                        _ => {
                            return Err("Expected identifier after 'buffer'".into());
                        }
                    };

                    let initial_value = match token_iter.next() {
                        Some(TokenType::Assign) => match token_iter.next() {
                            Some(TokenType::FloatLiteral(value)) => Expression::Literal(*value),
                            Some(TokenType::Identifier(name)) => {
                                Expression::Identifier(name.clone())
                            }
                            _ => return Err("Expected initial value after '='".into()),
                        },
                        _ => return Err("Expected default value after buffer identifier".into()),
                    };

                    let data_type = match token {
                        TokenType::Float => Type::Float,
                        TokenType::Buffer => Type::Buffer,
                        _ => unreachable!(),
                    };

                    tree.statements.push(Statement::VariableDeclaration(
                        VariableDeclarationStatement {
                            name,
                            data_type,
                            initial_value,
                        },
                    ));
                }

                TokenType::Identifier(name) => {
                    let target_name = name.clone();

                    if token_iter.next() != Some(&TokenType::Assign) {
                        return Err(format!("Expected '=' after identifier '{}'", name));
                    }

                    let value = match self.parse_expression(&mut token_iter) {
                        Ok(expr) => expr,
                        Err(e) => return Err(format!("Error parsing expression: {}", e)),
                    };

                    tree.statements
                        .push(Statement::Assignment(AssignmentStatement {
                            target_name,
                            value,
                        }));
                }

                TokenType::EndOfFile => {
                    // End of file, we can stop parsing
                    break;
                }

                _ => {}
            }
        }

        Ok(tree)
    }

    /// Parses an expression recursively from the token iterator.
    fn parse_expression(
        &self,
        token_iter: &mut std::iter::Peekable<std::slice::Iter<TokenType>>,
    ) -> Result<Expression, String> {
        let mut left = match token_iter.next() {
            Some(TokenType::FloatLiteral(value)) => Expression::Literal(*value),
            Some(TokenType::Identifier(name)) => Expression::Identifier(name.clone()),
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
