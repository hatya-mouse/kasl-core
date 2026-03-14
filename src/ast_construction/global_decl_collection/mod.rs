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

pub use resolvers::FuncDeclInfo;

use crate::{
    CompilationState, NameSpace, ParserDeclStmt,
    error::ErrorCollector,
    scope_manager::ScopeGraph,
    symbol_table::{FuncBodyMap, OpBodyMap},
    type_registry::StructGraph,
};

pub struct GlobalDeclCollector<'a> {
    ec: &'a mut ErrorCollector,
    name_space: &'a mut NameSpace,
    func_body_map: &'a mut FuncBodyMap,
    op_body_map: &'a mut OpBodyMap,
    comp_state: &'a mut CompilationState,

    scope_graph: &'a mut ScopeGraph,
    struct_graph: &'a mut StructGraph,
}

impl<'a> GlobalDeclCollector<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        name_space: &'a mut NameSpace,
        func_body_map: &'a mut FuncBodyMap,
        op_body_map: &'a mut OpBodyMap,
        comp_state: &'a mut CompilationState,
        scope_graph: &'a mut ScopeGraph,
        struct_graph: &'a mut StructGraph,
    ) -> Self {
        Self {
            ec,
            name_space,
            func_body_map,
            op_body_map,
            comp_state,
            scope_graph,
            struct_graph,
        }
    }

    pub fn process(&mut self, decl_stmts: &'a [ParserDeclStmt]) {
        for stmt in decl_stmts.iter() {
            self.process_stmt(stmt);
        }
    }
}
