//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

/// Creates `Function` instances from basic function informations.
mod func_builder;
/// Calls `build_func` from `func_builder` to create a `Function` and register it in the `FunctionContext`.
mod func_decl_resolver;
/// Resolves the default value and stores it in the global scope with the given kind of variable.
mod global_var_registrar;
/// Resolves import statements.
mod import_resolver;
/// Resolves operators and stores them in the global scope.
mod operator;
/// Resolves struct declarations and stores them in the global scope.
mod struct_type;
/// Resolves typealias statements.
mod typealias_resolver;
/// Calls `register_var_globally` from `global_var_registrar` to resolve variable declarations and store them in the global scope.
mod var_resolver;

use crate::{
    ast_nodes::symbol_table::FunctionType,
    parser::{ParserFuncParam, ParserScopeStmt, parser_ast::ParserTypeName},
};

pub struct FuncDeclInfo<'a> {
    pub func_type: FunctionType,
    pub name: &'a str,
    pub params: &'a [ParserFuncParam],
    pub return_type: &'a Option<ParserTypeName>,
    pub body: &'a [ParserScopeStmt],
}
