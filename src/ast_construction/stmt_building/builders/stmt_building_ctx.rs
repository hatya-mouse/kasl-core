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

use crate::{
    Program, SymbolTable, error::ErrorCollector, stmt_building::function_graph::FunctionGraphEdge,
};

pub struct StmtBuildingCtx<'a> {
    pub ec: &'a mut ErrorCollector,
    pub program: &'a mut Program,
    pub symbol_table: &'a SymbolTable<'a>,
    pub function_graph: Vec<FunctionGraphEdge>,
}

impl<'a> StmtBuildingCtx<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        program: &'a mut Program,
        symbol_table: &'a SymbolTable<'a>,
    ) -> Self {
        Self {
            ec,
            program,
            symbol_table,
            function_graph: Vec::new(),
        }
    }
}
