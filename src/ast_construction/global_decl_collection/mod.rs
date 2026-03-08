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

mod resolvers;
mod stmt_process;
mod struct_process;

use crate::{
    NameSpace, ParserDeclStmt,
    error::ErrorCollector,
    scope_manager::ScopeRegistry,
    symbol_table::{FunctionContext, OperatorContext},
    type_registry::TypeRegistry,
};

pub struct GlobalDeclCollector<'a> {
    ec: &'a mut ErrorCollector,
    decl_stmts: &'a [ParserDeclStmt],
    name_space: &'a mut NameSpace,
    type_registry: &'a mut TypeRegistry,
    func_ctx: &'a mut FunctionContext,
    op_ctx: &'a mut OperatorContext,
    scope_registry: &'a mut ScopeRegistry,
}

impl<'a> GlobalDeclCollector<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        decl_stmts: &'a [ParserDeclStmt],
        name_space: &'a mut NameSpace,
        type_registry: &'a mut TypeRegistry,
        func_ctx: &'a mut FunctionContext,
        op_ctx: &'a mut OperatorContext,
        scope_registry: &'a mut ScopeRegistry,
    ) -> Self {
        Self {
            ec,
            decl_stmts,
            name_space,
            type_registry,
            func_ctx,
            op_ctx,
            scope_registry,
        }
    }

    pub fn process(&mut self) {
        for stmt in self.decl_stmts.iter() {
            self.process_stmt(stmt);
        }
    }
}
