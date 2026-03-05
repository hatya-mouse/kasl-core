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

use crate::{ParserTopLevelStmt, SymbolPath, data::ParserStmtID};
use std::collections::HashMap;

/// SymbolTable stores a reference to the declaration statement (ParserTopLevelStmt) of variables, functions, operators, type definitions, and initializers.
#[derive(Debug, Clone)]
pub struct SymbolTable<'a> {
    statements: HashMap<ParserStmtID, &'a ParserTopLevelStmt>,
    /// Contains a mapping of symbol paths to their corresponding IDs.
    path_to_id: HashMap<SymbolPath, Vec<ParserStmtID>>,
    /// Contains a mapping of symbol IDs to their corresponding paths.
    id_to_path: HashMap<ParserStmtID, SymbolPath>,

    /// The next ID number.
    next_id: usize,
}

pub enum StatementLookup<'a> {
    Single(&'a ParserTopLevelStmt),
    Multiple(&'a [&'a ParserTopLevelStmt]),
}

impl<'a> Default for SymbolTable<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            statements: HashMap::new(),
            path_to_id: HashMap::new(),
            id_to_path: HashMap::new(),
            next_id: 0,
        }
    }

    /// Resolves a SymbolPath into a ParserStmtID.
    pub fn get_id_by_path(&self, path: &SymbolPath) -> Option<&Vec<ParserStmtID>> {
        self.path_to_id.get(path)
    }

    /// Returns a SymbolPath for the given ParserStmtID.
    pub fn get_path_by_id(&self, id: &ParserStmtID) -> Option<&SymbolPath> {
        self.id_to_path.get(id)
    }

    /// Checks if the symbol is already defined in the current scope.
    pub fn is_symbol_defined(&self, path: &SymbolPath) -> bool {
        self.path_to_id.contains_key(path)
    }

    /// Returns the next available SymbolID.
    pub fn next_id(&mut self) -> ParserStmtID {
        let id = self.next_id;
        self.next_id += 1;
        ParserStmtID::new(id)
    }

    /// Inserts a statement into the symbol table.
    pub fn insert_statement(&mut self, path: SymbolPath, stmt: &'a ParserTopLevelStmt) {
        let id = self.next_id();
        self.path_to_id.entry(path.clone()).or_default().push(id);
        self.id_to_path.insert(id, path);
        self.statements.insert(id, stmt);
    }

    /// Gets the statement by SymbolID.
    pub fn get_statement_by_id(&self, id: &ParserStmtID) -> Option<&&ParserTopLevelStmt> {
        self.statements.get(id)
    }

    /// Iterates over all statements and their corresponding SymbolIDs.
    pub fn get_tuples(&self) -> Vec<(ParserStmtID, &&ParserTopLevelStmt)> {
        self.statements
            .iter()
            .map(|(id, stmt)| (*id, stmt))
            .collect()
    }

    /// Iterates over all statements.
    pub fn get_statements(&self) -> Vec<&&ParserTopLevelStmt> {
        self.statements.values().collect()
    }
}
