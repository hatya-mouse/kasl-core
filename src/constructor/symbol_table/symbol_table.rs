//
// Copyright 2025-2026 Shuntaro Kasatani
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

use crate::{ParserStatement, ParserSymbolPath, SymbolPath, SymbolPathComponent};
use std::collections::HashMap;

/// SymbolTable stores a reference to the declaration statement (ParserStatement) of variables, functions, operators, type definitions, and initializers.
#[derive(Debug, Clone)]
pub struct SymbolTable<'a> {
    pub inputs: HashMap<String, &'a ParserStatement>,
    pub outputs: HashMap<String, &'a ParserStatement>,
    pub states: HashMap<String, &'a ParserStatement>,
    pub vars: HashMap<String, &'a ParserStatement>,
    pub funcs: HashMap<String, &'a ParserStatement>,
    pub type_defs: HashMap<String, (&'a ParserStatement, SymbolTable<'a>)>,
    // pub infix_defines: HashMap<String, &'a ParserStatement>,
    // pub prefix_defines: HashMap<String, &'a ParserStatement>,
    // pub infix_funcs: HashMap<String, &'a ParserStatement>,
    // pub prefix_funcs: HashMap<String, &'a ParserStatement>,
    pub inits: Vec<&'a ParserStatement>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            states: HashMap::new(),
            vars: HashMap::new(),
            funcs: HashMap::new(),
            type_defs: HashMap::new(),
            infix_defines: HashMap::new(),
            prefix_defines: HashMap::new(),
            infix_funcs: HashMap::new(),
            prefix_funcs: HashMap::new(),
            inits: Vec::new(),
        }
    }

    /// Resolves a ParserSymbolPath into a SymbolPath.
    pub fn resolve_path(&self, parser_path: &ParserSymbolPath) -> Option<SymbolPath> {
        let mut result_path = SymbolPath::new();
        let mut current_scope = self;

        for component in parser_path {
            if &component.symbol == "CompInt" {
                result_path.push(SymbolPathComponent::CompInt);
            } else if &component.symbol == "CompFloat" {
                result_path.push(SymbolPathComponent::CompFloat);
            } else if &component.symbol == "CompBool" {
                result_path.push(SymbolPathComponent::CompBool);
            } else if let Some(_) = current_scope.get_input(&component.symbol) {
                result_path.push(SymbolPathComponent::InputVar(component.symbol.clone()));
            } else if let Some(_) = current_scope.get_output(&component.symbol) {
                result_path.push(SymbolPathComponent::OutputVar(component.symbol.clone()));
            } else if let Some(_) = current_scope.get_state(&component.symbol) {
                result_path.push(SymbolPathComponent::StateVar(component.symbol.clone()));
            } else if let Some(_) = current_scope.get_var(&component.symbol) {
                result_path.push(SymbolPathComponent::Var(component.symbol.clone()));
            } else if let Some(_) = current_scope.get_func(&component.symbol) {
                result_path.push(SymbolPathComponent::Func(component.symbol.clone()));
            } else if let Some(type_def) = current_scope.get_type_def(&component.symbol) {
                result_path.push(SymbolPathComponent::TypeDef(component.symbol.clone()));
                current_scope = &type_def.1;
            } else {
                return None;
            }
        }

        Some(result_path)
    }

    // Insert functions

    pub fn insert_input(&mut self, name: String, stmt: &'a ParserStatement) {
        self.inputs.insert(name, stmt);
    }

    pub fn insert_output(&mut self, name: String, stmt: &'a ParserStatement) {
        self.outputs.insert(name, stmt);
    }

    pub fn insert_state(&mut self, name: String, stmt: &'a ParserStatement) {
        self.states.insert(name, stmt);
    }

    pub fn insert_var(&mut self, name: String, stmt: &'a ParserStatement) {
        self.vars.insert(name, stmt);
    }

    pub fn insert_func(&mut self, name: String, stmt: &'a ParserStatement) {
        self.funcs.insert(name, stmt);
    }

    pub fn insert_type_def(
        &mut self,
        name: String,
        stmt: &'a ParserStatement,
        sub_table: SymbolTable<'a>,
    ) {
        self.type_defs.insert(name, (stmt, sub_table));
    }

    pub fn insert_infix_define(&mut self, symbol: String, stmt: &'a ParserStatement) {
        self.infix_defines.insert(symbol, stmt);
    }

    pub fn insert_prefix_define(&mut self, symbol: String, stmt: &'a ParserStatement) {
        self.prefix_defines.insert(symbol, stmt);
    }

    pub fn insert_infix_func(&mut self, symbol: String, stmt: &'a ParserStatement) {
        self.infix_funcs.insert(symbol, stmt);
    }

    pub fn insert_prefix_func(&mut self, symbol: String, stmt: &'a ParserStatement) {
        self.prefix_funcs.insert(symbol, stmt);
    }

    pub fn insert_init(&mut self, stmt: &'a ParserStatement) {
        self.inits.push(stmt);
    }

    // Getter functions

    pub fn get_input(&self, name: &str) -> Option<&&ParserStatement> {
        self.inputs.get(name)
    }

    pub fn get_output(&self, name: &str) -> Option<&&ParserStatement> {
        self.outputs.get(name)
    }

    pub fn get_state(&self, name: &str) -> Option<&&ParserStatement> {
        self.states.get(name)
    }

    pub fn get_var(&self, name: &str) -> Option<&&ParserStatement> {
        self.vars.get(name)
    }

    pub fn get_func(&self, name: &str) -> Option<&&ParserStatement> {
        self.funcs.get(name)
    }

    pub fn get_type_def(&self, name: &str) -> Option<&(&ParserStatement, SymbolTable<'a>)> {
        self.type_defs.get(name)
    }

    pub fn get_infix_define(&self, symbol: &str) -> Option<&&ParserStatement> {
        self.infix_defines.get(symbol)
    }

    pub fn get_prefix_define(&self, symbol: &str) -> Option<&&ParserStatement> {
        self.prefix_defines.get(symbol)
    }

    pub fn get_infix_func(&self, symbol: &str) -> Option<&&ParserStatement> {
        self.infix_funcs.get(symbol)
    }

    pub fn get_prefix_func(&self, symbol: &str) -> Option<&&ParserStatement> {
        self.prefix_funcs.get(symbol)
    }

    pub fn get_inits(&self) -> &Vec<&ParserStatement> {
        &self.inits
    }

    /// Gets the statement by SymbolPath.
    /// Componenets except the last one must be a Type statement.
    pub fn get_statement_by_path(&self, symbol_path: &SymbolPath) -> Option<&&ParserStatement> {
        let mut current_scope = self;
        let last_index = symbol_path.components.len().checked_sub(1)?;

        for i in 0..last_index {
            match &symbol_path.components[i] {
                SymbolPathComponent::TypeDef(type_name) => {
                    if let Some(type_def_entry) = current_scope.type_defs.get(type_name) {
                        current_scope = &type_def_entry.1;
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }

        match &symbol_path.components[last_index] {
            SymbolPathComponent::InputVar(name) => current_scope.get_input(name),
            SymbolPathComponent::OutputVar(name) => current_scope.get_output(name),
            SymbolPathComponent::StateVar(name) => current_scope.get_state(name),
            SymbolPathComponent::Var(name) => current_scope.get_var(name),
            SymbolPathComponent::Func(name) => current_scope.get_func(name),
            SymbolPathComponent::TypeDef(name) => {
                current_scope.get_type_def(name).map(|entry| &entry.0)
            }
            _ => None,
        }
    }
}
