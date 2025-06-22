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

use crate::TokenType;

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input }
    }

    pub fn tokenize(&self) -> Vec<TokenType> {
        let mut tokens = Vec::new();
        for line in self.input.lines() {
            tokens.extend(self.tokenize_line(line));
        }
        tokens.push(TokenType::EndOfFile); // Add EOF token at the end
        tokens
    }

    fn tokenize_line(&self, line: &str) -> Vec<TokenType> {
        let mut tokens = Vec::new();
        let mut chars = line.chars().peekable();

        while let Some(char) = chars.next() {
            match char {
                ' ' | '\t' | '\n' => continue, // Skip whitespace
                '+' => tokens.push(TokenType::Plus),
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
                                tokens.push(TokenType::FloatLiteral(value));
                            }
                        } else {
                            tokens.push(TokenType::Minus);
                        }
                    } else {
                        tokens.push(TokenType::Minus);
                    }
                }
                '*' => tokens.push(TokenType::Multiply),
                '/' => {
                    if chars.peek() == Some(&'/') {
                        // Skip comment
                        while chars.next().is_some() && chars.peek() != Some(&'\n') {}
                        tokens.push(TokenType::Comment);
                    } else {
                        tokens.push(TokenType::Divide);
                    }
                }
                '%' => tokens.push(TokenType::Modulo),
                '=' => tokens.push(TokenType::Assign),
                '(' => tokens.push(TokenType::LParen),
                ')' => tokens.push(TokenType::RParen),
                ',' => tokens.push(TokenType::Comma),
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
                        tokens.push(TokenType::FloatLiteral(value));
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
                        "input" => tokens.push(TokenType::Input),
                        "output" => tokens.push(TokenType::Output),
                        "float" => tokens.push(TokenType::Float),
                        "buffer" => tokens.push(TokenType::Buffer),
                        _ => tokens.push(TokenType::Identifier(identifier)),
                    }
                }
                _ => {} // Ignore unrecognized characters
            }
        }

        tokens.push(TokenType::EndOfLine); // End of line token
        tokens
    }
}
