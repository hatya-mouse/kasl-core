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

use std::collections::HashMap;

use crate::ParserStatement;

#[derive(Debug, Clone)]
pub struct SymbolTable<'a> {
    pub vars: HashMap<String, &'a ParserStatement>,
    pub funcs: HashMap<String, &'a ParserStatement>,
    pub operators: HashMap<String, &'a ParserStatement>,
    pub type_defs: HashMap<String, (&'a ParserStatement, SymbolTable<'a>)>,
    pub inits: Vec<&'a ParserStatement>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            operators: HashMap::new(),
            type_defs: HashMap::new(),
            inits: Vec::new(),
        }
    }

    pub fn insert_var(&mut self, name: String, stmt: &'a ParserStatement) {
        self.vars.insert(name, stmt);
    }

    pub fn insert_func(&mut self, name: String, stmt: &'a ParserStatement) {
        self.funcs.insert(name, stmt);
    }

    pub fn insert_operator(&mut self, name: String, stmt: &'a ParserStatement) {
        self.operators.insert(name, stmt);
    }

    pub fn insert_type_def(
        &mut self,
        name: String,
        stmt: &'a ParserStatement,
        sub_table: SymbolTable<'a>,
    ) {
        self.type_defs.insert(name, (stmt, sub_table));
    }

    pub fn insert_init(&mut self, stmt: &'a ParserStatement) {
        self.inits.push(stmt);
    }

    pub fn get_var(&self, name: &str) -> Option<&&ParserStatement> {
        self.vars.get(name)
    }

    pub fn get_func(&self, name: &str) -> Option<&&ParserStatement> {
        self.funcs.get(name)
    }

    pub fn get_operator(&self, name: &str) -> Option<&&ParserStatement> {
        self.operators.get(name)
    }

    pub fn get_type_def(&self, name: &str) -> Option<&(&ParserStatement, SymbolTable<'a>)> {
        self.type_defs.get(name)
    }

    pub fn get_inits(&self) -> &Vec<&ParserStatement> {
        &self.inits
    }
}
