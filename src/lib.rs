pub(crate) const MAIN_FUNCTION_NAME: &str = "main";

pub mod ast;
pub mod ast_construction;
pub mod backend;
pub mod builtin;
pub mod compiler;
pub mod localization;
pub mod parser;

pub use ast::*;
pub use ast_construction::*;
pub use compiler::KaslCompiler;
pub use parser::*;
