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

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Input,
    Output,
    Var,
    For,
    In,
    // Types
    Number,
    // Brackets
    LBracket,
    RBracket,
    // Braces
    LBrace,
    RBrace,
    // Operators
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    // Literals
    FloatLiteral(f32),
    // Identifiers
    Identifier(String),
    // Delimiters
    LParen,
    RParen,
    Comma,
    // Special tokens
    EndOfLine,
    EndOfFile,
    Comment,
}
