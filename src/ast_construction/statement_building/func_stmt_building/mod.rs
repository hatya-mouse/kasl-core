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
    FunctionID, NameSpace, OperatorContext, ScopeRegistry,
    error::ErrorCollector,
    scope_manager::ScopeGraph,
    symbol_table::{FuncBodyMap, FunctionContext},
    type_registry::{ResolvedType, TypeRegistry},
};

pub struct FuncStmtBuilder<'a> {
    ec: &'a mut ErrorCollector,
    name_space: &'a mut NameSpace,
    type_registry: &'a TypeRegistry,
    func_ctx: &'a mut FunctionContext,
    func_body_map: &'a FuncBodyMap,
    op_ctx: &'a OperatorContext,
    scope_registry: &'a mut ScopeRegistry,

    scope_graph: &'a mut ScopeGraph,
    func_id: FunctionID,
    expected_return_type: Option<ResolvedType>,
}

impl<'a> FuncStmtBuilder<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        name_space: &'a mut NameSpace,
        type_registry: &'a TypeRegistry,
        func_ctx: &'a mut FunctionContext,
        func_body_map: &'a FuncBodyMap,
        op_ctx: &'a OperatorContext,
        scope_registry: &'a mut ScopeRegistry,
        scope_graph: &'a mut ScopeGraph,
        func_id: FunctionID,
    ) -> Self {
        let func = func_ctx.get_func(&func_id);
        let expected_return_type = func.and_then(|f| f.return_type.clone());

        Self {
            ec,
            name_space,
            type_registry,
            func_ctx,
            func_body_map,
            op_ctx,
            scope_registry,
            scope_graph,
            func_id,
            expected_return_type,
        }
    }
}
