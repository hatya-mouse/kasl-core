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

mod func_stmt_building;

pub use func_stmt_building::FuncStmtBuilder;

use crate::{
    CompilationState, NameSpace, error::ErrorCollector, scope_manager::ScopeGraph,
    symbol_table::FuncBodyMap,
};

pub struct StatementBuilder<'a> {
    ec: &'a mut ErrorCollector,
    name_space: &'a mut NameSpace,
    func_body_map: &'a FuncBodyMap,
    compilation_state: &'a mut CompilationState,

    scope_graph: &'a mut ScopeGraph,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        name_space: &'a mut NameSpace,
        func_body_map: &'a FuncBodyMap,
        compilation_state: &'a mut CompilationState,
        scope_graph: &'a mut ScopeGraph,
    ) -> Self {
        Self {
            ec,
            name_space,
            func_body_map,
            compilation_state,
            scope_graph,
        }
    }

    pub fn build_all(&mut self) {
        for func_id in self.compilation_state.func_ctx.func_ids() {
            let mut func_stmt_builder = FuncStmtBuilder::new(
                self.ec,
                self.name_space,
                self.func_body_map,
                self.compilation_state,
                self.scope_graph,
                func_id,
            );
            func_stmt_builder.build_func_body();
        }
    }
}
