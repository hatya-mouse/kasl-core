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
    pub type_defs: HashMap<String, &'a ParserStatement>,

    /// Path to the initializers are just a path to the
    /// Path example: "Hoge"
    pub inits: HashMap<String, Vec<&'a ParserStatement>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            operators: HashMap::new(),
            type_defs: HashMap::new(),
            inits: HashMap::new(),
        }
    }

    pub fn insert_var(&mut self, path: String, stmt: &'a ParserStatement) {
        self.vars.insert(path, stmt);
    }

    pub fn insert_func(&mut self, path: String, stmt: &'a ParserStatement) {
        self.funcs.insert(path, stmt);
    }

    pub fn insert_operator(&mut self, path: String, stmt: &'a ParserStatement) {
        self.operators.insert(path, stmt);
    }

    pub fn insert_type_def(&mut self, path: String, stmt: &'a ParserStatement) {
        self.operators.insert(path, stmt);
    }

    pub fn insert_init(&mut self, struct_name: String, stmt: &'a ParserStatement) {
        self.inits
            .entry(struct_name)
            .or_insert_with(Vec::new)
            .push(stmt);
    }

    pub fn get_var(&self, path: &str) -> Option<&&ParserStatement> {
        self.vars.get(path)
    }

    pub fn get_func(&self, path: &str) -> Option<&&ParserStatement> {
        self.funcs.get(path)
    }

    pub fn get_operator(&self, path: &str) -> Option<&&ParserStatement> {
        self.operators.get(path)
    }

    pub fn get_type_def(&self, path: &str) -> Option<&&ParserStatement> {
        self.type_defs.get(path)
    }

    pub fn get_inits(&self, path: &str) -> Option<&Vec<&ParserStatement>> {
        self.inits.get(path)
    }
}
