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

use crate::{Function, Program, Statement, SymbolInfo, SymbolKind};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SemanticAnalyzer {
    pub symbol_table: HashMap<String, SymbolInfo>,
    pub var_table: HashMap<String, SymbolInfo>,
    pub input_table: HashMap<String, SymbolInfo>,
    pub output_table: HashMap<String, SymbolInfo>,
    pub function_table: HashMap<String, Function>,
    errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: HashMap::new(),
            var_table: HashMap::new(),
            input_table: HashMap::new(),
            output_table: HashMap::new(),
            function_table: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<String>> {
        self.function_table
            .extend(crate::builtin_function::built_in_functions());

        for statement in &program.statements {
            match statement {
                Statement::InputDeclaration(input) => {
                    let name = input.name.clone();
                    let data_type = input.data_type.clone();
                    let initial_value = input.initial_value.clone();

                    self.define_input(
                        name.clone(),
                        SymbolInfo {
                            name: name.clone(),
                            kind: SymbolKind::Input,
                            data_type,
                            initial_value,
                            range: None,
                            value: None,
                        },
                    );
                }

                Statement::OutputDeclaration(output) => {
                    let name = output.name.clone();
                    let data_type = output.data_type.clone();

                    self.define_output(
                        name.clone(),
                        SymbolInfo {
                            name: name.clone(),
                            kind: SymbolKind::Output,
                            data_type,
                            initial_value: None,
                            range: None,
                            value: None,
                        },
                    );
                }

                Statement::VariableDeclaration(var) => {
                    let name = var.name.clone();
                    let data_type = var.data_type.clone();

                    self.define_symbol(
                        name.clone(),
                        SymbolInfo {
                            name: name.clone(),
                            kind: SymbolKind::Variable,
                            data_type,
                            initial_value: None,
                            range: None,
                            value: None,
                        },
                    );
                }

                Statement::Assignment(assignment) => {
                    let target = &assignment.target_name;
                    let value = &assignment.value;

                    if let Some(info) = self.symbol_table.get(target) {
                        let target_type = info.data_type.clone();
                        let value_type = match value.get_expression_type(
                            &self.symbol_table,
                            &self.function_table,
                            Some(target_type),
                        ) {
                            Ok(t) => t,
                            Err(e) => {
                                self.errors
                                    .push(format!("Error in assignment to '{}': {}", target, e));
                                continue;
                            }
                        };

                        if target_type != value_type {
                            self.errors.push(format!(
                                "Type mismatch in assignment to '{}': expected {:?}, found {:?}.",
                                target, target_type, value_type
                            ));
                        }
                    } else {
                        self.errors.push(format!("Undefined symbol '{}'.", target));
                    }
                }
            }
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    pub fn get_input_table(&self) -> HashMap<String, SymbolInfo> {
        self.input_table.clone()
    }

    pub fn get_output_table(&self) -> HashMap<String, SymbolInfo> {
        self.output_table.clone()
    }

    fn define_symbol(&mut self, name: String, info: SymbolInfo) {
        if self.symbol_table.contains_key(&name) {
            self.errors
                .push(format!("Symbol '{}' is already defined.", name));
        } else {
            self.symbol_table.insert(name.clone(), info.clone());
            self.var_table.insert(name, info);
        }
    }

    fn define_input(&mut self, name: String, info: SymbolInfo) {
        if self.input_table.contains_key(&name) {
            self.errors
                .push(format!("UI parameter '{}' is already defined.", name));
        } else {
            self.symbol_table.insert(name.clone(), info.clone());
            self.input_table.insert(name, info);
        }
    }

    fn define_output(&mut self, name: String, info: SymbolInfo) {
        if self.output_table.contains_key(&name) {
            self.errors
                .push(format!("UI parameter '{}' is already defined.", name));
        } else {
            self.symbol_table.insert(name.clone(), info.clone());
            self.output_table.insert(name, info);
        }
    }
}
