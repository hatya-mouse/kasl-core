use std::collections::HashMap;

use crate::{Expression, Program, Statement, Type};

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Time,
    Input,
    Output,
    Variable,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub kind: SymbolKind,
    pub data_type: Type,
    pub initial_value: Option<Expression>,
    pub range: Option<(f64, f64)>,
}

#[derive(Debug)]
pub struct SemanticAnalyzer {
    pub symbol_table: HashMap<String, SymbolInfo>,
    pub ui_params: HashMap<String, SymbolInfo>,
    errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: HashMap::new(),
            ui_params: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<String>> {
        self.define_built_in_symbols();

        for statement in &program.statements {
            match statement {
                Statement::InputDeclaration(input) => {
                    let name = input.name.clone();
                    let data_type = input.data_type.clone();
                    let initial_value = input.initial_value.clone();

                    self.define_ui_param(
                        name,
                        SymbolInfo {
                            kind: SymbolKind::Input,
                            data_type,
                            initial_value: Some(initial_value),
                            range: None,
                        },
                    );
                }

                Statement::OutputDeclaration(output) => {
                    let name = output.name.clone();
                    let data_type = output.data_type.clone();

                    self.define_ui_param(
                        name,
                        SymbolInfo {
                            kind: SymbolKind::Output,
                            data_type,
                            initial_value: None,
                            range: None,
                        },
                    );
                }

                Statement::VariableDeclaration(var) => {
                    let name = var.name.clone();
                    let data_type = var.data_type.clone();

                    self.define_symbol(
                        name,
                        SymbolInfo {
                            kind: SymbolKind::Variable,
                            data_type,
                            initial_value: None,
                            range: None,
                        },
                    );
                }

                Statement::Assignment(assignment) => {
                    let target = &assignment.target_name;
                    let value = &assignment.value;

                    if let Some(info) = self.symbol_table.get(target) {
                        let target_type = info.data_type.clone();
                        let value_type = self.get_expression_type(value);

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

    fn get_expression_type(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::BinaryOp { left, right, .. } => {
                let left_type = self.get_expression_type(left);
                let right_type = self.get_expression_type(right);
                if left_type == right_type {
                    left_type
                } else {
                    match (&left_type, &right_type) {
                        (Type::Float, Type::Buffer) => Type::Buffer,
                        (Type::Buffer, Type::Float) => Type::Buffer,
                        _ => {
                            self.errors.push(format!(
                                "Type mismatch: {:?} and {:?} cannot be combined.",
                                left_type, right_type
                            ));
                            Type::Float // Default to Float for error handling
                        }
                    }
                }
            }
            Expression::Literal(_) => Type::Float,
            Expression::Identifier(name) => self
                .symbol_table
                .get(name)
                .map_or(Type::Float, |info| info.data_type.clone()),
        }
    }

    fn define_built_in_symbols(&mut self) {
        self.symbol_table.insert(
            "TIME".to_string(),
            SymbolInfo {
                kind: SymbolKind::Time,
                data_type: Type::Float,
                initial_value: None,
                range: None,
            },
        );
    }

    fn define_symbol(&mut self, name: String, info: SymbolInfo) {
        if self.symbol_table.contains_key(&name) {
            self.errors
                .push(format!("Symbol '{}' is already defined.", name));
        } else {
            self.symbol_table.insert(name, info);
        }
    }

    fn define_ui_param(&mut self, name: String, info: SymbolInfo) {
        if self.ui_params.contains_key(&name) {
            self.errors
                .push(format!("UI parameter '{}' is already defined.", name));
        } else {
            self.ui_params.insert(name, info);
        }
    }
}
