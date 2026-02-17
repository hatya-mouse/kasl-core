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
    ParserStatement, Program, SymbolTable,
    error::{ErrorCollector, ErrorRecord},
    resolution::type_resolver::resolve_types,
    table_construction::build_symbol_table,
    type_collection::collect_all_types,
    validation::validator::validate,
};

/// Process order:
/// 1. `symbol_table`: Build symbol table
/// 2. `type_collection`: Collect types
/// 3. `validation`: Validate program
/// 6. `resolve_types`: Resolve types and construct the AST
pub fn construct_program(statements: Vec<ParserStatement>) -> Result<(), Vec<ErrorRecord>> {
    let mut program = Program::new();
    let mut symbol_table = SymbolTable::new();
    let mut error_collector = ErrorCollector::new();

    // 1. Build symbol table
    build_symbol_table(&mut error_collector, &mut symbol_table, &statements);

    // 2. Collect types
    collect_all_types(&mut program, &symbol_table);

    // 3. Validate program
    validate(&mut error_collector, &symbol_table);

    // 4. Infer the types of symbols
    resolve_types(&mut error_collector, &mut program, &symbol_table);

    error_collector.as_result()
}
