//
// © 2025-2026 Shuntaro Kasatani
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

mod expr_builder;
mod expr_resolver;
mod resolvers;
mod stmt_process;
mod struct_process;

use crate::{
    NameSpace, ParserDeclStmt,
    error::ErrorCollector,
    global_decl_collection::expr_builder::ExpressionBuilder,
    scope_manager::ScopeRegistry,
    symbol_table::{FunctionContext, OperatorContext},
    type_registry::TypeRegistry,
};

pub struct GlobalDeclCollector<'a> {
    pub ec: &'a mut ErrorCollector,
    pub decl_stmts: &'a [ParserDeclStmt],
    pub name_space: &'a mut NameSpace,
    pub type_registry: &'a mut TypeRegistry,
    pub function_ctx: &'a mut FunctionContext,
    pub operator_ctx: &'a mut OperatorContext,
    pub scope_registry: &'a mut ScopeRegistry,

    expression_builder: &'a mut ExpressionBuilder,
}

impl GlobalDeclCollector<'_> {
    pub fn process(&mut self) {
        for stmt in self.decl_stmts.iter() {
            self.process_stmt(stmt);
        }
    }
}
