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
    ParserStatement, Program, ResolverError, SymbolTable,
    member_collection::collect_all_type_members, symbol_collection::collect_top_level_symbols,
    type_collection::collect_types,
};

pub fn resolve(statements: Vec<ParserStatement>) -> Result<(), ResolverError> {
    let mut program = Program::new();
    let mut symbol_table = SymbolTable::new();

    program.types = collect_types(&statements);
    collect_top_level_symbols(&mut program, &mut symbol_table, &statements)?;
    collect_all_type_members(&mut program, &mut symbol_table, &statements)?;

    Ok(())
}
