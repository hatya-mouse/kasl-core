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
    Expression, Function, Operator, Program, Statement, SymbolInfo, SymbolKind, builtin_function,
};
use knodiq_engine::{Sample, Value};
use std::collections::HashMap;

pub struct Interpreter {
    pub program: Program,
    pub symbol_table: HashMap<String, SymbolInfo>,
    pub function_table: HashMap<String, Function>,

    sample_rate: usize,
    channels: usize,
    chunk_start: usize,
    chunk_end: usize,
}

impl Interpreter {
    pub fn new(
        program: Program,
        sample_rate: usize,
        channels: usize,
        chunk_start: usize,
        chunk_end: usize,
    ) -> Self {
        Interpreter {
            program,
            symbol_table: HashMap::new(),
            function_table: HashMap::new(),
            sample_rate,
            channels,
            chunk_start,
            chunk_end,
        }
    }

    fn initialize(&mut self) {
        self.symbol_table.clear();
        self.function_table.clear();

        self.function_table
            .extend(builtin_function::built_in_functions());
    }

    pub fn execute(
        &mut self,
        input_parameters: HashMap<String, SymbolInfo>,
    ) -> Result<HashMap<String, SymbolInfo>, String> {
        self.initialize();

        let statements = self.program.statements.clone();

        self.execute_statements(&statements, input_parameters)?;

        let output_table = self
            .symbol_table
            .iter()
            .filter_map(|(name, info)| {
                if info.kind == SymbolKind::Output {
                    Some((name.clone(), info.clone()))
                } else {
                    None
                }
            })
            .collect::<HashMap<String, SymbolInfo>>();
        Ok(output_table)
    }

    fn execute_statements(
        &mut self,
        statements: &Vec<Statement>,
        input_parameters: HashMap<String, SymbolInfo>,
    ) -> Result<(), String> {
        for statement in statements {
            match &statement {
                Statement::Assignment(assignment) => {
                    let value = self.evaluate_expression(&assignment.value)?;
                    let target = self
                        .symbol_table
                        .get(&assignment.target_name)
                        .cloned()
                        .ok_or_else(|| {
                            format!("Variable '{}' not found", assignment.target_name)
                        })?;
                    let symbol = SymbolInfo {
                        name: target.name.clone(),
                        kind: target.kind,
                        initial_value: target.initial_value.clone(),
                        range: target.range,
                        value: Some(value.clone()),
                    };
                    self.symbol_table
                        .insert(assignment.target_name.clone(), symbol.clone());
                }

                Statement::InputDeclaration(input) => {
                    if let Some(input_param) = input_parameters.get(&input.name).cloned() {
                        let symbol = SymbolInfo {
                            name: input.name.clone(),
                            kind: SymbolKind::Input,
                            initial_value: None,
                            range: None,
                            value: Some(input_param.value.unwrap_or(Value::Float(0.0))),
                        };
                        self.register_symbol(input.name.clone(), symbol.clone())?;
                    } else {
                        return Err(format!("Input parameter '{}' not provided", input.name));
                    }
                }

                Statement::OutputDeclaration(output) => {
                    let symbol = SymbolInfo {
                        name: output.name.clone(),
                        kind: SymbolKind::Output,
                        initial_value: None,
                        range: None,
                        value: None,
                    };
                    self.register_symbol(output.name.clone(), symbol.clone())?;
                }

                Statement::VariableDeclaration(var_decl) => {
                    let initial_value = self.evaluate_expression(&var_decl.initial_value)?;
                    let symbol = SymbolInfo {
                        name: var_decl.name.clone(),
                        kind: SymbolKind::Variable,
                        initial_value: None,
                        range: None,
                        value: Some(initial_value),
                    };
                    self.register_symbol(var_decl.name.clone(), symbol.clone())?;
                }

                Statement::ForLoop(loop_stmt) => {
                    let symbol = SymbolInfo {
                        name: loop_stmt.variable_name.clone(),
                        kind: SymbolKind::Variable,
                        initial_value: None,
                        range: None,
                        value: Some(Value::Float(0.0)),
                    };
                    self.register_symbol(loop_stmt.variable_name.clone(), symbol.clone())?;

                    let iterable = self.evaluate_expression(&loop_stmt.iterable)?;
                    match iterable {
                        Value::Array(elements) => {
                            for element in elements {
                                let symbol_info = SymbolInfo {
                                    name: loop_stmt.variable_name.clone(),
                                    kind: SymbolKind::Variable,
                                    initial_value: None,
                                    range: None,
                                    value: Some(element.clone()),
                                };
                                self.symbol_table
                                    .insert(loop_stmt.variable_name.clone(), symbol_info);

                                self.execute_statements(&loop_stmt.body, input_parameters.clone())?;
                            }
                        }
                        Value::Float(value) => {
                            let symbol_info = SymbolInfo {
                                name: loop_stmt.variable_name.clone(),
                                kind: SymbolKind::Variable,
                                initial_value: None,
                                range: None,
                                value: Some(Value::Float(value)),
                            };
                            self.symbol_table
                                .insert(loop_stmt.variable_name.clone(), symbol_info);

                            self.execute_statements(&loop_stmt.body, input_parameters.clone())?;
                        }
                    }

                    // Clean up the loop variable after execution
                    self.symbol_table.remove(&loop_stmt.variable_name);
                }
            }
        }

        Ok(())
    }

    fn register_symbol(&mut self, name: String, info: SymbolInfo) -> Result<(), String> {
        if self.symbol_table.contains_key(&name) {
            Err(format!("Symbol '{}' is already defined.", name))
        } else {
            self.symbol_table.insert(name.clone(), info);
            Ok(())
        }
    }

    fn evaluate_expression(&self, expression: &Expression) -> Result<Value, String> {
        match expression {
            Expression::Literal(value) => Ok(Value::Float(*value)),

            Expression::Identifier(name) => self
                .symbol_table
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Variable '{}' not found", name))?
                .value
                .ok_or_else(|| format!("Variable '{}' has no value", name)),

            Expression::BinaryOp { left, op, right } => {
                let left_value = self.evaluate_expression(left)?;
                let right_value = self.evaluate_expression(right)?;

                let evaluated_value = match op {
                    Operator::Add => left_value.apply_op(&right_value, Box::new(|a, b| a + b)),
                    Operator::Subtract => left_value.apply_op(&right_value, Box::new(|a, b| a - b)),
                    Operator::Multiply => left_value.apply_op(&right_value, Box::new(|a, b| a * b)),
                    Operator::Divide => {
                        if let Value::Float(0.0) = right_value {
                            Some(Value::Float(0.0))
                        } else {
                            left_value.apply_op(&right_value, Box::new(|a, b| a / b))
                        }
                    }
                    Operator::Modulo => {
                        if let Value::Float(0.0) = right_value {
                            Some(Value::Float(0.0))
                        } else {
                            left_value.apply_op(&right_value, Box::new(|a, b| a % b))
                        }
                    }
                };

                evaluated_value.ok_or_else(|| {
                    format!(
                        "Invalid operation: {:?} {:?} {:?}",
                        left_value, op, right_value
                    )
                })
            }

            Expression::FunctionCall { name, arguments } => {
                let func = match self.function_table.get(name) {
                    Some(func) => func,
                    None => return Err(format!("Function '{}' not found", name)),
                };

                match self.evaluate_function(&func, arguments) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(format!("Error evaluating function '{}': {}", name, e)),
                }
            }
        }
    }

    fn evaluate_function(&self, func: &Function, args: &Vec<Expression>) -> Result<Value, String> {
        let mut evaluated_args = Vec::new();
        for arg in args {
            let arg_value = self.evaluate_expression(arg)?;
            evaluated_args.push(arg_value);
        }

        match func.name.as_str() {
            "sin" => Ok(evaluated_args[0]
                .apply_fn(|x| x.sin())
                .ok_or("Arguments are invalid for sin.")?),
            "cos" => Ok(evaluated_args[0]
                .apply_fn(|x| x.cos())
                .ok_or("Arguments are invalid for cos.")?),
            "tan" => Ok(evaluated_args[0]
                .apply_fn(|x| x.tan())
                .ok_or("Arguments are invalid for tan.")?),
            "asin" => Ok(evaluated_args[0]
                .apply_fn(|x| x.asin())
                .ok_or("Arguments are invalid for asin.")?),
            "acos" => Ok(evaluated_args[0]
                .apply_fn(|x| x.acos())
                .ok_or("Arguments are invalid for acos.")?),
            "atan" => Ok(evaluated_args[0]
                .apply_fn(|x| x.atan())
                .ok_or("Arguments are invalid for atan.")?),
            "abs" => Ok(evaluated_args[0]
                .apply_fn(|x| x.abs())
                .ok_or("Arguments are invalid for abs.")?),
            "sgn" => Ok(evaluated_args[0]
                .apply_fn(|x| x.signum())
                .ok_or("Arguments are invalid for sgn.")?),
            "min" => Ok(evaluated_args[0]
                .apply_op(&evaluated_args[1], |a, b| a.min(b))
                .ok_or("Arguments are invalid for min.")?),
            "max" => Ok(evaluated_args[0]
                .apply_op(&evaluated_args[1], |a, b| a.max(b))
                .ok_or("Arguments are invalid for max.")?),
            "clamp" => Ok(evaluated_args[0]
                .apply_op(&evaluated_args[1], |a, min| a.max(min))
                .ok_or("Arguments are invalid for clamp.")?
                .apply_op(&evaluated_args[2], |a, max| a.min(max))
                .ok_or("Arguments are invalid for clamp.")?),
            "pow" => Ok(evaluated_args[0]
                .apply_op(&evaluated_args[1], |a, b| a.powf(b))
                .ok_or("Arguments are invalid for pow.")?),
            "sqrt" => Ok(evaluated_args[0]
                .apply_fn(|x| x.sqrt())
                .ok_or("Arguments are invalid for sqrt.")?),
            "log" => Ok(evaluated_args[0]
                .apply_fn(|x| x.ln())
                .ok_or("Arguments are invalid for log (ln).")?),
            "log2" => Ok(evaluated_args[0]
                .apply_fn(|x| x.log2())
                .ok_or("Arguments are invalid for log2.")?),
            "log10" => Ok(evaluated_args[0]
                .apply_fn(|x| x.log10())
                .ok_or("Arguments are invalid for log10.")?),
            "saw" => Ok(evaluated_args[0]
                .apply_fn(|x| {
                    let phase = x % 1.0;
                    2.0 * (phase - 0.5)
                })
                .ok_or("Arguments are invalid for saw.")?),
            "tri" => Ok(evaluated_args[0]
                .apply_fn(|x| {
                    let phase = x % 1.0;
                    if phase < 0.5 {
                        4.0 * phase - 1.0
                    } else {
                        1.0 - (phase - 0.5) * 4.0
                    }
                })
                .ok_or("Arguments are invalid for tri.")?),
            "square" => Ok(evaluated_args[0]
                .apply_fn(|x| {
                    let phase = x % 1.0;
                    if phase < 0.5 { 1.0 } else { -1.0 }
                })
                .ok_or("Arguments are invalid for square.")?),
            "rand" => Ok(Value::Float(rand::random::<f32>())),
            "mix" => {
                if evaluated_args.len() != 3 {
                    return Err("mix function requires exactly 3 arguments".to_string());
                }
                if let Value::Float(factor) = evaluated_args[2] {
                    Ok(evaluated_args[0]
                        .apply_op(&evaluated_args[1], |a, b| a * (1.0 - factor) + b * factor)
                        .ok_or("Arguments are invalid for mix.")?)
                } else {
                    Err("Third argument for mix must be a float".to_string())
                }
            }
            "lerp" => {
                if evaluated_args.len() != 3 {
                    return Err("lerp function requires exactly 3 arguments".to_string());
                }
                if let Value::Float(factor) = evaluated_args[2] {
                    Ok(evaluated_args[0]
                        .apply_op(&evaluated_args[1], |a, b| a * (1.0 - factor) + b * factor)
                        .ok_or("Arguments are invalid for lerp.")?)
                } else {
                    Err("Third argument for lerp must be a float".to_string())
                }
            }
            "pi" => Ok(Value::Float(std::f32::consts::PI)),
            "time" => Ok(Value::from_buffer(
                (0..self.channels)
                    .map(|_| {
                        (self.chunk_start..self.chunk_end)
                            .map(|t| t as Sample / self.sample_rate as Sample)
                            .collect()
                    })
                    .collect(),
            )),
            "sample_rate" => Ok(Value::Float(self.sample_rate as Sample)),
            _ => Err(format!("Unknown function '{}'", func.name)),
        }
    }
}
