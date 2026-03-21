pub mod grammar;
pub mod parser_ast;

pub use grammar::kasl_parser;
pub use parser_ast::{
    ExprToken, ExprTokenKind, ParserDeclStmt, ParserDeclStmtKind, ParserFuncCallArg,
    ParserFuncParam, ParserIfArm, ParserInputAttribute, ParserOperatorType, ParserProgram,
    ParserScopeStmt, ParserScopeStmtKind,
};
