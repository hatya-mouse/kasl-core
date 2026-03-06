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
    ParserTopLevelStmt, Program, SymbolTable,
    error::{ErrorCollector, ErrorRecord},
    resolution::type_resolver::resolve_types,
    stmt_building::build_statements,
    table_construction::build_symbol_table,
    validation::validator::validate,
};

pub fn construct_program(statements: Vec<ParserTopLevelStmt>) -> Result<(), Vec<ErrorRecord>> {
    let mut program = Program::new();
    let mut symbol_table = SymbolTable::new();
    let mut error_collector = ErrorCollector::new();

    // 1. Build symbol table
    build_symbol_table(&mut error_collector, &mut symbol_table, &statements);

    // 2. Resolve types and construct the AST
    resolve_types(&mut error_collector, &mut program, &symbol_table);

    // 3. Build the function bodies
    build_statements(&mut error_collector, &mut program, &symbol_table);

    // 4. Validate program
    validate(&mut error_collector, &program);

    error_collector.as_result()
}
