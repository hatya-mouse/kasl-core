/// Creates `Function` instances from basic function informations.
mod func_builder;
/// Calls `build_func` from `func_builder` to create a `Function` and register it in the `FunctionContext`.
mod func_decl_resolver;
/// Resolves the default value and stores it in the global scope with the given kind of variable.
mod global_var_registrar;
/// Resolves import statements.
mod import_resolver;
/// Resolves operators and stores them in the global scope.
mod op_resolver;
/// Resolves struct declarations and stores them in the global scope.
mod struct_resolver;
/// Resolves typealias statements.
mod typealias_resolver;
/// Calls `register_var_globally` from `global_var_registrar` to resolve variable declarations and store them in the global scope.
mod var_resolver;

use crate::{
    ParserFuncParam, ParserScopeStmt, parser_ast::ParserTypeName, symbol_table::FunctionType,
};

pub struct FuncDeclInfo<'a> {
    pub func_type: FunctionType,
    pub name: &'a str,
    pub params: &'a [ParserFuncParam],
    pub return_type: &'a Option<ParserTypeName>,
    pub body: &'a [ParserScopeStmt],
}
