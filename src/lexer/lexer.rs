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

use crate::{TokenType, token_type::Token};

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let lines = self.input.lines();
        for (line_number, line) in lines.enumerate() {
            tokens.extend(self.tokenize_line(line, line_number));
        }
        tokens.push(Token {
            token_type: TokenType::EndOfFile,
            line: self.input.lines().count(),
        });
        tokens
    }

    fn tokenize_line(&self, line: &str, line_number: usize) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = line.chars().peekable();

        while let Some(char) = chars.next() {
            match char {
                ' ' | '\t' | '\n' => continue, // Skip whitespace
                '+' => tokens.push(Token {
                    token_type: TokenType::Plus,
                    line: self.input.lines().count(),
                }),
                '-' => {
                    // If the next character is a digit, it should be a negative number
                    if let Some(&next_char) = chars.peek() {
                        if next_char.is_digit(10) {
                            let mut number = String::from("-");
                            while let Some(&next_char) = chars.peek() {
                                if next_char.is_digit(10) || next_char == '.' {
                                    number.push(chars.next().unwrap());
                                } else {
                                    break;
                                }
                            }
                            if let Ok(value) = number.parse::<f32>() {
                                tokens.push(Token {
                                    token_type: TokenType::FloatLiteral(value),
                                    line: line_number,
                                });
                            }
                        } else {
                            tokens.push(Token {
                                token_type: TokenType::Minus,
                                line: line_number,
                            });
                        }
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::Minus,
                            line: line_number,
                        });
                    }
                }
                '*' => tokens.push(Token {
                    token_type: TokenType::Multiply,
                    line: line_number,
                }),
                '/' => {
                    if chars.peek() == Some(&'/') {
                        // Skip comment
                        while chars.next().is_some() && chars.peek() != Some(&'\n') {}
                        tokens.push(Token {
                            token_type: TokenType::Comment,
                            line: line_number,
                        });
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::Divide,
                            line: line_number,
                        });
                    }
                }
                '%' => tokens.push(Token {
                    token_type: TokenType::Modulo,
                    line: line_number,
                }),
                '=' => tokens.push(Token {
                    token_type: TokenType::Assign,
                    line: line_number,
                }),
                '(' => tokens.push(Token {
                    token_type: TokenType::LParen,
                    line: line_number,
                }),
                ')' => tokens.push(Token {
                    token_type: TokenType::RParen,
                    line: line_number,
                }),
                ',' => tokens.push(Token {
                    token_type: TokenType::Comma,
                    line: line_number,
                }),
                '[' => tokens.push(Token {
                    token_type: TokenType::LBracket,
                    line: line_number,
                }),
                ']' => tokens.push(Token {
                    token_type: TokenType::RBracket,
                    line: line_number,
                }),
                '{' => tokens.push(Token {
                    token_type: TokenType::LBrace,
                    line: line_number,
                }),
                '}' => tokens.push(Token {
                    token_type: TokenType::RBrace,
                    line: line_number,
                }),
                '0'..='9' => {
                    let mut number = char.to_string();
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_digit(10) || next_char == '.' {
                            number.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    if let Ok(value) = number.parse::<f32>() {
                        tokens.push(Token {
                            token_type: TokenType::FloatLiteral(value),
                            line: line_number,
                        });
                    }
                }
                _ if char.is_alphabetic() || char == '_' => {
                    let mut identifier = char.to_string();
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            identifier.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    match identifier.as_str() {
                        "input" => tokens.push(Token {
                            token_type: TokenType::Input,
                            line: line_number,
                        }),
                        "output" => tokens.push(Token {
                            token_type: TokenType::Output,
                            line: line_number,
                        }),
                        "var" => tokens.push(Token {
                            token_type: TokenType::Var,
                            line: line_number,
                        }),
                        "number" => tokens.push(Token {
                            token_type: TokenType::Number,
                            line: line_number,
                        }),
                        "for" => tokens.push(Token {
                            token_type: TokenType::For,
                            line: line_number,
                        }),
                        "in" => tokens.push(Token {
                            token_type: TokenType::In,
                            line: line_number,
                        }),
                        _ => tokens.push(Token {
                            token_type: TokenType::Identifier(identifier),
                            line: line_number,
                        }),
                    }
                }
                _ => {} // Ignore unrecognized characters
            }
        }

        tokens.push(Token {
            token_type: TokenType::EndOfLine,
            line: line_number,
        });
        tokens
    }
}
