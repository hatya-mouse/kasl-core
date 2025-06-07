#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Input,
    Output,
    InputRange(f32, f32),
    // Types
    Float,
    Buffer,
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
