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

mod builders;
mod stmt_builder;

use crate::{
    ScopeRegistry,
    error::ErrorCollector,
    symbol_table::{FuncBodyMap, FunctionContext},
};

pub struct StatementBuilder<'a> {
    ec: &'a mut ErrorCollector,
    func_ctx: &'a mut FunctionContext,
    func_body_map: &'a FuncBodyMap,
    scope_registry: &'a mut ScopeRegistry,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        func_ctx: &'a mut FunctionContext,
        func_body_map: &'a FuncBodyMap,
        scope_registry: &'a mut ScopeRegistry,
    ) -> Self {
        Self {
            ec,
            func_ctx,
            func_body_map,
            scope_registry,
        }
    }

    pub fn build_stmts(&mut self) {
        for func_id in self.func_ctx.func_ids() {
            self.build_stmt_for_func(func_id);
        }
    }
}
