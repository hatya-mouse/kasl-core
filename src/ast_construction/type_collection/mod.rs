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

mod stmt_process;

use crate::{NameSpace, ParserDeclStmt, error::ErrorCollector, type_registry::TypeRegistry};

pub struct TypeCollector<'a> {
    ec: &'a mut ErrorCollector,
    decl_stmts: &'a [ParserDeclStmt],
    name_space: &'a mut NameSpace,
    type_registry: &'a mut TypeRegistry,
}

impl<'a> TypeCollector<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        decl_stmts: &'a [ParserDeclStmt],
        name_space: &'a mut NameSpace,
        type_registry: &'a mut TypeRegistry,
    ) -> Self {
        Self {
            ec,
            decl_stmts,
            name_space,
            type_registry,
        }
    }

    pub fn process(&mut self) {
        for stmt in self.decl_stmts.iter() {
            self.process_stmt(stmt);
        }
    }
}
