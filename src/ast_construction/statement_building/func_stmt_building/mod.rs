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
/// Builds a Block which contains ScopeID from a list of statements.
mod scope_block_builder;
mod stmt_builder;

use crate::{
    CompilationState, FunctionID, NameSpace, error::ErrorCollector, scope_manager::ScopeGraph,
    symbol_table::FuncBodyMap, type_registry::ResolvedType,
};

pub struct FuncStmtBuilder<'a> {
    ec: &'a mut ErrorCollector,
    name_space: &'a mut NameSpace,
    func_body_map: &'a FuncBodyMap,
    compilation_state: &'a mut CompilationState,

    scope_graph: &'a mut ScopeGraph,
    func_id: FunctionID,
    expected_return_type: Option<ResolvedType>,
}

impl<'a> FuncStmtBuilder<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ec: &'a mut ErrorCollector,
        name_space: &'a mut NameSpace,
        func_body_map: &'a FuncBodyMap,
        compilation_state: &'a mut CompilationState,
        scope_graph: &'a mut ScopeGraph,
        func_id: FunctionID,
    ) -> Self {
        let func = compilation_state.func_ctx.get_func(&func_id);
        let expected_return_type = func.and_then(|f| f.return_type);

        Self {
            ec,
            name_space,
            func_body_map,
            compilation_state,
            scope_graph,
            func_id,
            expected_return_type,
        }
    }
}
