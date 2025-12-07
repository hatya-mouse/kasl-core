//
// Copyright 2025 Shuntaro Kasatani
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
    ConstructorError, ParserStatement, Program, SymbolTable,
    member_collection::collect_all_type_members, resolution::resolver::resolve_types,
    symbol_collection::collect_top_level_symbols, symbol_table::build_symbol_table,
    type_collection::collect_all_types,
};

/// Process order:
/// 1. `symbol_table`: Build symbol table
/// 2. `type_collection`: Collect types
/// 3. `symbol_collection`: Collect top-level symbols
/// 4. `member_collection`: Collect all type members
/// 5. `type_resolution`: Resolve types
pub fn construct_program(statements: Vec<ParserStatement>) -> Result<(), Vec<ConstructorError>> {
    let mut program = Program::new();
    let mut symbol_table = SymbolTable::new();

    // 1. Build symbol table
    build_symbol_table(&mut symbol_table, &statements);

    // 2. Collect types
    collect_all_types(&mut program, &symbol_table);

    // 3. Collect top-level symbols
    collect_top_level_symbols(&mut program, &symbol_table).map_err(|err| vec![err])?;

    // 4. Collect all type members
    collect_all_type_members(&mut program, &symbol_table).map_err(|err| vec![err])?;

    // 5. Resolve types
    resolve_types(&mut program, &symbol_table)?;

    Ok(())
}
