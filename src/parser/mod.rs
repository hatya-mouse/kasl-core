pub mod ast;
pub mod parser;

pub use ast::{
    AssignmentStatement, Expression, InputDeclarationStatement, Operator,
    OutputDeclarationStatement, Program, Statement, Type, VariableDeclarationStatement,
};
pub use parser::Parser;
