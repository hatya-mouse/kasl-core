#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Input,
    Output,
    InputRange(f64, f64),
    // Types
    Buffer,
    Float,
    // Operators
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    // Literals
    FloatLiteral(f64),
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
