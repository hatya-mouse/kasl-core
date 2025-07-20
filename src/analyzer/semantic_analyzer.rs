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

use knodiq_engine::{Type, graph::value::type_of};

use crate::{
    Expression, Function, Operator, Program, SemanticError, Statement, SymbolInfo, SymbolKind,
    VariableDeclarationStatement,
    semantic_error::{self, ErrorVariant},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SemanticAnalyzer {
    pub symbol_table: HashMap<String, SymbolInfo>,
    pub var_table: HashMap<String, SymbolInfo>,
    pub input_table: Vec<SymbolInfo>,
    pub output_table: Vec<SymbolInfo>,
    pub function_table: HashMap<String, Function>,
    error: SemanticError,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: HashMap::new(),
            var_table: HashMap::new(),
            input_table: Vec::new(),
            output_table: Vec::new(),
            function_table: HashMap::new(),
            error: SemanticError::new(),
        }
    }

    /// Analyzes the given program for semantic errors.
    /// Also infers variable types and checks for undefined symbols.
    pub fn analyze(&mut self, program: &Program) -> Result<Program, SemanticError> {
        let mut program = program.clone();
        Ok(Program {
            statements: self.analyze_statements(&mut program.statements)?,
        })
    }

    pub fn analyze_statements(
        &mut self,
        statements: &mut Vec<Statement>,
    ) -> Result<Vec<Statement>, SemanticError> {
        self.function_table
            .extend(crate::builtin_function::built_in_functions());
        let mut result = Vec::new();

        for statement in statements {
            match statement {
                Statement::InputDeclaration(input) => {
                    let name = input.name.clone();

                    self.define_input(
                        name.clone(),
                        SymbolInfo {
                            name: name.clone(),
                            kind: SymbolKind::Input,
                            value_type: input.value_type.clone(),
                            value: None,
                        },
                    );

                    result.push(Statement::InputDeclaration(input.clone()));
                }

                Statement::OutputDeclaration(output) => {
                    let name = output.name.clone();

                    self.define_output(
                        name.clone(),
                        SymbolInfo {
                            name: name.clone(),
                            kind: SymbolKind::Output,
                            value_type: output.value_type.clone(),
                            value: None,
                        },
                    );

                    result.push(Statement::OutputDeclaration(output.clone()));
                }

                Statement::VariableDeclaration(var) => {
                    let name = var.name.clone();

                    let value_type = self.infer_type(&var.initial_value)?;

                    self.define_symbol(
                        name.clone(),
                        SymbolInfo {
                            name: name.clone(),
                            kind: SymbolKind::Variable,
                            value_type: Type::Float,
                            value: None,
                        },
                    );

                    result.push(Statement::VariableDeclaration(
                        VariableDeclarationStatement {
                            name: name.clone(),
                            value_type,
                            initial_value: var.initial_value.clone(),
                            line: var.line,
                        },
                    ));
                }

                Statement::Assignment(assignment) => {
                    let target = &assignment.target_name;

                    if !self.symbol_table.contains_key(target) {
                        self.error
                            .errors
                            .push(semantic_error::ErrorVariant::UndefinedSymbol(
                                target.clone(),
                            ));
                    }

                    if let Err(e) = self.evaluate_expression(&mut assignment.value) {
                        self.error.errors.extend(e.errors);
                    }

                    result.push(Statement::Assignment(assignment.clone()));
                }

                Statement::ForLoop(loop_stmt) => {
                    let variable_name = &loop_stmt.variable_name;

                    if self.symbol_table.contains_key(variable_name) {
                        self.error
                            .errors
                            .push(semantic_error::ErrorVariant::SymbolAlreadyDefined(
                                variable_name.clone(),
                            ));
                    } else {
                        self.define_symbol(
                            variable_name.clone(),
                            SymbolInfo {
                                name: variable_name.clone(),
                                kind: SymbolKind::Variable,
                                value_type: Type::Int,
                                value: None,
                            },
                        );
                    }

                    // Check if the body is valid
                    if let Err(e) = self.analyze_statements(&mut loop_stmt.body) {
                        self.error.errors.extend(e.errors);
                    }

                    result.push(Statement::ForLoop(loop_stmt.clone()));
                }
            }
        }

        if self.error.errors.is_empty() {
            Ok(result)
        } else {
            Err(self.error.clone())
        }
    }

    pub fn evaluate_expression(&mut self, expr: &Expression) -> Result<Expression, SemanticError> {
        match expr {
            Expression::IntLiteral(_) => Ok(expr.clone()),
            Expression::FloatLiteral(_) => Ok(expr.clone()),
            Expression::Identifier(name) => {
                if !self.symbol_table.contains_key(name) {
                    self.error
                        .errors
                        .push(semantic_error::ErrorVariant::UndefinedSymbol(name.clone()));
                }
                Ok(expr.clone())
            }
            Expression::FunctionCall { name, arguments } => {
                if !self.function_table.contains_key(name) {
                    self.error
                        .errors
                        .push(semantic_error::ErrorVariant::UndefinedFunction(
                            name.clone(),
                        ));
                }
                let mut evaluated_arguments = Vec::new();
                for arg in arguments {
                    evaluated_arguments.push(self.evaluate_expression(arg)?);
                }
                Ok(Expression::FunctionCall {
                    name: name.clone(),
                    arguments: evaluated_arguments,
                })
            }
            Expression::BinaryOp {
                op,
                left,
                right,
                left_type: _,
                right_type: _,
            } => {
                let evaluated_left = Box::new(self.evaluate_expression(left)?);
                let evaluated_right = Box::new(self.evaluate_expression(right)?);

                let left_type = self.infer_type(&left)?;
                let right_type = self.infer_type(&right)?;

                if op == &Operator::Modulo && (left_type != Type::Int || right_type != Type::Int) {
                    self.error
                        .errors
                        .push(semantic_error::ErrorVariant::InvalidOperation(
                            Operator::Modulo,
                            left_type.clone(),
                            right_type.clone(),
                        ));
                }

                Ok(Expression::BinaryOp {
                    op: op.clone(),
                    left: evaluated_left,
                    right: evaluated_right,
                    left_type,
                    right_type,
                })
            }
        }
    }

    pub fn get_inputs(&self) -> Vec<SymbolInfo> {
        self.input_table.clone()
    }

    pub fn get_outputs(&self) -> Vec<SymbolInfo> {
        self.output_table.clone()
    }

    fn define_symbol(&mut self, name: String, info: SymbolInfo) {
        if self.symbol_table.contains_key(&name) {
            self.error
                .errors
                .push(semantic_error::ErrorVariant::SymbolAlreadyDefined(
                    name.clone(),
                ));
        } else {
            self.symbol_table.insert(name.clone(), info.clone());
            self.var_table.insert(name, info);
        }
    }

    fn define_input(&mut self, name: String, info: SymbolInfo) {
        if self.input_table.iter().any(|sym| sym.name == name) {
            self.error
                .errors
                .push(semantic_error::ErrorVariant::SymbolAlreadyDefined(
                    name.clone(),
                ));
        } else {
            self.symbol_table.insert(name.clone(), info.clone());
            self.input_table.push(info);
        }
    }

    fn define_output(&mut self, name: String, info: SymbolInfo) {
        if self.output_table.iter().any(|sym| sym.name == name) {
            self.error
                .errors
                .push(semantic_error::ErrorVariant::SymbolAlreadyDefined(
                    name.clone(),
                ));
        } else {
            self.symbol_table.insert(name.clone(), info.clone());
            self.output_table.push(info);
        }
    }

    fn infer_type(&self, expr: &Expression) -> Result<Type, SemanticError> {
        match expr {
            Expression::IntLiteral(_) => Ok(Type::Int),
            Expression::FloatLiteral(_) => Ok(Type::Float),
            Expression::Identifier(name) => {
                if let Some(info) = self.symbol_table.get(name) {
                    Ok(info.value_type.clone())
                } else {
                    Err(SemanticError {
                        errors: vec![ErrorVariant::UndefinedSymbol(name.clone())],
                    })
                }
            }
            Expression::FunctionCall { name, arguments } => {
                if let Some(_) = self.function_table.get(name) {
                    let input_types = arguments.iter().map(|arg| self.infer_type(arg));
                    let mut inferred_type = Type::None;
                    for arg in input_types {
                        match arg {
                            Ok(t) => inferred_type = type_of(&inferred_type, &t),
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(inferred_type)
                } else {
                    return Err(SemanticError {
                        errors: vec![ErrorVariant::UndefinedFunction(name.clone())],
                    });
                }
            }
            Expression::BinaryOp {
                op: _,
                left: _,
                right: _,
                left_type,
                right_type,
            } => Ok(type_of(&left_type, &right_type)),
        }
    }
}
