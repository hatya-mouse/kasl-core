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
    Expression, Function, Operator, Program, RuntimeError, Statement, SymbolInfo, SymbolKind,
    builtin_function,
};
use knodiq_engine::{
    Sample, Value,
    audio_utils::{beats_as_samples, samples_as_beats},
};
use std::collections::HashMap;

pub struct Interpreter {
    pub program: Program,
    pub symbol_table: HashMap<String, SymbolInfo>,
    pub function_table: HashMap<String, Function>,

    sample_rate: usize,
    samples_per_beat: f32,
    channels: usize,
    chunk_start: usize,
    chunk_end: usize,
}

impl Interpreter {
    pub fn new(
        program: Program,
        sample_rate: usize,
        samples_per_beat: f32,
        channels: usize,
        chunk_start: usize,
        chunk_end: usize,
    ) -> Self {
        Interpreter {
            program,
            symbol_table: HashMap::new(),
            function_table: HashMap::new(),
            sample_rate,
            samples_per_beat,
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
    ) -> Result<HashMap<String, SymbolInfo>, RuntimeError> {
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
    ) -> Result<(), RuntimeError> {
        for statement in statements {
            match &statement {
                Statement::Assignment(assignment) => {
                    let value = self.evaluate_expression(&assignment.value, assignment.line)?;
                    let target = self
                        .symbol_table
                        .get(&assignment.target_name)
                        .cloned()
                        .ok_or_else(|| RuntimeError::SymbolNotFound {
                            name: assignment.target_name.clone(),
                            line: assignment.line,
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
                        self.register_symbol(input.name.clone(), symbol.clone(), input.line)?;
                    } else {
                        return Err(RuntimeError::InputNotProvided {
                            name: input.name.clone(),
                            line: input.line,
                        });
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
                    self.register_symbol(output.name.clone(), symbol.clone(), output.line)?;
                }

                Statement::VariableDeclaration(var_decl) => {
                    let initial_value =
                        self.evaluate_expression(&var_decl.initial_value, var_decl.line)?;
                    let symbol = SymbolInfo {
                        name: var_decl.name.clone(),
                        kind: SymbolKind::Variable,
                        initial_value: None,
                        range: None,
                        value: Some(initial_value),
                    };
                    self.register_symbol(var_decl.name.clone(), symbol.clone(), var_decl.line)?;
                }

                Statement::ForLoop(loop_stmt) => {
                    let symbol = SymbolInfo {
                        name: loop_stmt.variable_name.clone(),
                        kind: SymbolKind::Variable,
                        initial_value: None,
                        range: None,
                        value: Some(Value::Float(0.0)),
                    };
                    self.register_symbol(
                        loop_stmt.variable_name.clone(),
                        symbol.clone(),
                        loop_stmt.line,
                    )?;

                    let iterable = self.evaluate_expression(&loop_stmt.iterable, loop_stmt.line)?;
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

    fn register_symbol(
        &mut self,
        name: String,
        info: SymbolInfo,
        line: usize,
    ) -> Result<(), RuntimeError> {
        if self.symbol_table.contains_key(&name) {
            Err(RuntimeError::SymbolAlreadyDefined { name, line })
        } else {
            self.symbol_table.insert(name.clone(), info);
            Ok(())
        }
    }

    fn evaluate_expression(
        &self,
        expression: &Expression,
        line: usize,
    ) -> Result<Value, RuntimeError> {
        match expression {
            Expression::Literal(value) => Ok(Value::Float(*value)),

            Expression::Identifier(name) => self
                .symbol_table
                .get(name)
                .cloned()
                .ok_or_else(|| RuntimeError::SymbolNotFound {
                    name: name.clone(),
                    line,
                })?
                .value
                .ok_or_else(|| RuntimeError::Unknown { line }),

            Expression::BinaryOp { left, op, right } => {
                let left_value = self.evaluate_expression(left, line)?;
                let right_value = self.evaluate_expression(right, line)?;

                let evaluated_value = match op {
                    Operator::Add => left_value + right_value,
                    Operator::Subtract => left_value - right_value,
                    Operator::Multiply => left_value * right_value,
                    Operator::Divide => left_value / right_value,
                    Operator::Modulo => left_value % right_value,
                };

                Ok(evaluated_value)
            }

            Expression::FunctionCall { name, arguments } => {
                let func = match self.function_table.get(name) {
                    Some(func) => func,
                    None => {
                        return Err(RuntimeError::FunctionNotFound {
                            name: name.clone(),
                            line,
                        });
                    }
                };

                self.evaluate_function(&func, arguments, line)
            }
        }
    }

    fn evaluate_function(
        &self,
        func: &Function,
        args: &Vec<Expression>,
        line: usize,
    ) -> Result<Value, RuntimeError> {
        let mut evaluated_args = Vec::new();
        for arg in args {
            let arg_value = self.evaluate_expression(arg, line)?;
            evaluated_args.push(arg_value);
        }

        let inv_arg_err = |name: &str| RuntimeError::FunctionInvalidArgumentError {
            name: name.to_string(),
            line,
        };

        match func.name.as_str() {
            "sin" => Ok(Value::apply_op(&[&evaluated_args[0]], |args| args[0].sin())
                .ok_or(inv_arg_err("sin"))?),
            "cos" => Ok(Value::apply_op(&[&evaluated_args[0]], |args| args[0].cos())
                .ok_or(inv_arg_err("cos"))?),
            "tan" => Ok(Value::apply_op(&[&evaluated_args[0]], |args| args[0].tan())
                .ok_or(inv_arg_err("tan"))?),
            "asin" => Ok(
                Value::apply_op(&[&evaluated_args[0]], |args| args[0].asin())
                    .ok_or(inv_arg_err("asin"))?,
            ),
            "acos" => Ok(
                Value::apply_op(&[&evaluated_args[0]], |args| args[0].acos())
                    .ok_or(inv_arg_err("acos"))?,
            ),
            "atan" => Ok(
                Value::apply_op(&[&evaluated_args[0]], |args| args[0].atan())
                    .ok_or(inv_arg_err("atan"))?,
            ),
            "abs" => Ok(Value::apply_op(&[&evaluated_args[0]], |args| args[0].abs())
                .ok_or(inv_arg_err("abs"))?),
            "sgn" => Ok(
                Value::apply_op(&[&evaluated_args[0]], |args| args[0].signum())
                    .ok_or(inv_arg_err("sgn"))?,
            ),
            "min" => Ok(
                Value::apply_op(&[&evaluated_args[0], &evaluated_args[1]], |args| {
                    args[0].min(args[1])
                })
                .ok_or(inv_arg_err("min"))?,
            ),
            "max" => Ok(
                Value::apply_op(&[&evaluated_args[0], &evaluated_args[1]], |args| {
                    args[0].max(args[1])
                })
                .ok_or(inv_arg_err("max"))?,
            ),
            "clamp" => Ok(Value::apply_op(
                &[
                    &Value::apply_op(&[&evaluated_args[0], &evaluated_args[1]], |args| {
                        args[0].max(args[1])
                    })
                    .ok_or(inv_arg_err("clamp"))?,
                    &evaluated_args[2],
                ],
                |args| args[0].min(args[1]),
            )
            .ok_or(inv_arg_err("clamp"))?),
            "pow" => Ok(
                Value::apply_op(&[&evaluated_args[0], &evaluated_args[1]], |args| {
                    args[0].powf(args[1])
                })
                .ok_or(inv_arg_err("pow"))?,
            ),
            "sqrt" => Ok(
                Value::apply_op(&[&evaluated_args[0]], |args| args[0].sqrt())
                    .ok_or(inv_arg_err("sqrt"))?,
            ),
            "log" => Ok(Value::apply_op(&[&evaluated_args[0]], |args| args[0].ln())
                .ok_or(inv_arg_err("log"))?),
            "log2" => Ok(
                Value::apply_op(&[&evaluated_args[0]], |args| args[0].log2())
                    .ok_or(inv_arg_err("log2"))?,
            ),
            "log10" => Ok(
                Value::apply_op(&[&evaluated_args[0]], |args| args[0].log10())
                    .ok_or(inv_arg_err("log10"))?,
            ),
            "saw" => Ok(Value::apply_op(&[&evaluated_args[0]], |args| {
                let phase = args[0] % 1.0;
                2.0 * (phase - 0.5)
            })
            .ok_or(inv_arg_err("saw"))?),
            "tri" => Ok(Value::apply_op(&[&evaluated_args[0]], |args| {
                let phase = args[0] % 1.0;
                if phase < 0.5 {
                    4.0 * phase - 1.0
                } else {
                    1.0 - (phase - 0.5) * 4.0
                }
            })
            .ok_or(inv_arg_err("tri"))?),
            "square" => Ok(Value::apply_op(&[&evaluated_args[0]], |args| {
                let phase = args[0] % 1.0;
                if phase < 0.5 { 1.0 } else { -1.0 }
            })
            .ok_or(inv_arg_err("square"))?),
            "rand" => Ok(Value::Float(rand::random::<Sample>())),
            "mix" => {
                if evaluated_args.len() != 3 {
                    return Err(inv_arg_err("mix"));
                }
                Ok(Value::apply_op(
                    &[&evaluated_args[0], &evaluated_args[1], &evaluated_args[2]],
                    |args| args[0] * (1.0 - args[2]) + args[1] * args[2],
                )
                .ok_or(inv_arg_err("mix"))?)
            }
            "lerp" => {
                if evaluated_args.len() != 3 {
                    return Err(inv_arg_err("lerp"));
                }
                Ok(Value::apply_op(
                    &[&evaluated_args[0], &evaluated_args[1], &evaluated_args[2]],
                    |args| args[0] * (1.0 - args[2]) + args[1] * args[2],
                )
                .ok_or(inv_arg_err("lerp"))?)
            }
            "load_time" => {
                if evaluated_args.len() != 2 {
                    return Err(inv_arg_err("load_time"));
                }

                // The first argument should be the audio buffer, and the second should be the time
                let audio = match &evaluated_args[0] {
                    Value::Array(v) => v,
                    _ => return Err(inv_arg_err("load_time")),
                };

                let time = match &evaluated_args[1] {
                    Value::Array(v) => {
                        match v.len() {
                            2 => v.clone(), // Use as is if two values are provided
                            _ => return Err(inv_arg_err("load_beats")),
                        }
                    }
                    _ => return Err(inv_arg_err("load_beats")),
                };

                // Convert time values to sample indices
                let time_sample = time
                    .iter()
                    .map(|v| match v {
                        Value::Float(t) => (t * self.sample_rate as Sample).floor() as usize,
                        _ => 0,
                    })
                    .collect::<Vec<usize>>();

                Ok(get_samples_between(audio, &time_sample[0], &time_sample[1]))
            }
            "load_beats" => {
                if evaluated_args.len() != 2 {
                    return Err(inv_arg_err("load_beats"));
                }

                // The first argument should be the audio buffer, and the second should be the time
                let audio = match &evaluated_args[0] {
                    Value::Array(v) => v,
                    _ => return Err(inv_arg_err("load_beats")),
                };

                let time = match &evaluated_args[1] {
                    Value::Array(v) => {
                        match v.len() {
                            2 => v.clone(), // Use as is if two values are provided
                            _ => return Err(inv_arg_err("load_beats")),
                        }
                    }
                    _ => return Err(inv_arg_err("load_beats")),
                };

                // Convert beats to sample indices
                let time_sample = time
                    .iter()
                    .map(|v| match v {
                        Value::Float(t) => beats_as_samples(self.samples_per_beat, *t),
                        _ => 0,
                    })
                    .collect::<Vec<usize>>();

                Ok(get_samples_between(audio, &time_sample[0], &time_sample[1]))
            }
            "pi" => Ok(Value::Float(std::f32::consts::PI)),
            "time" => Ok(Value::Array(
                // (self.chunk_start..self.chunk_end)
                //     .map(|t| Value::Float(t as Sample / self.sample_rate as Sample))
                //     .collect(),
                vec![
                    Value::Float(self.chunk_start as Sample / self.sample_rate as Sample),
                    Value::Float(self.chunk_end as Sample / self.sample_rate as Sample),
                ],
            )),
            "beats" => Ok(Value::Array(vec![
                Value::Float(samples_as_beats(self.samples_per_beat, self.chunk_start)),
                Value::Float(samples_as_beats(self.samples_per_beat, self.chunk_end)),
            ])),
            "sample_rate" => Ok(Value::Float(self.sample_rate as Sample)),
            "channels" => Ok(Value::Float(self.channels as Sample)),
            _ => Err(RuntimeError::FunctionNotFound {
                name: func.name.clone(),
                line: line,
            }),
        }
    }
}

fn get_samples_between(audio: &Vec<Value>, start: &usize, end: &usize) -> Value {
    Value::Array(
        audio
            .iter()
            .skip(*start)
            .take(end - start + 1)
            .cloned()
            .collect(),
    )
}
