use crate::{
    Expression, FunctionInfo, Operator, Program, Statement, SymbolInfo, SymbolKind, Type, Value,
    function,
};
use std::collections::HashMap;
pub struct Interpreter {
    pub program: Program,
    pub symbol_table: HashMap<String, SymbolInfo>,
    pub var_table: HashMap<String, SymbolInfo>,
    pub input_table: HashMap<String, SymbolInfo>,
    pub output_table: HashMap<String, SymbolInfo>,
    pub function_table: HashMap<String, FunctionInfo>,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Interpreter {
            program,
            symbol_table: HashMap::new(),
            var_table: HashMap::new(),
            input_table: HashMap::new(),
            output_table: HashMap::new(),
            function_table: HashMap::new(),
        }
    }

    fn initialize(&mut self) {
        self.symbol_table.clear();
        self.var_table.clear();
        self.input_table.clear();
        self.output_table.clear();
        self.function_table.clear();

        self.function_table.extend(function::built_in_functions());
    }

    pub fn execute(&mut self, input_parameters: HashMap<String, SymbolInfo>) -> Result<(), String> {
        self.initialize();

        for statement in &self.program.statements {
            match &statement {
                Statement::Assignment(assignment) => {
                    let value = self.evaluate_expression(&assignment.value)?;
                    let symbol = SymbolInfo {
                        name: assignment.target_name.clone(),
                        kind: SymbolKind::Variable,
                        data_type: assignment
                            .value
                            .get_expression_type(&self.symbol_table, &self.function_table)?,
                        initial_value: None,
                        range: None,
                        value: Some(value),
                    };
                    self.symbol_table
                        .insert(assignment.target_name.clone(), symbol.clone());
                    self.var_table
                        .insert(assignment.target_name.clone(), symbol.clone());
                }

                Statement::InputDeclaration(input) => {
                    if let Some(input_param) = input_parameters.get(&input.name).cloned() {
                        let symbol = SymbolInfo {
                            name: input.name.clone(),
                            kind: SymbolKind::Input,
                            data_type: input.data_type.clone(),
                            initial_value: None,
                            range: None,
                            value: Some(input_param.value.unwrap_or(Value::Float(0.0))),
                        };
                        self.symbol_table.insert(input.name.clone(), symbol.clone());
                        self.input_table.insert(input.name.clone(), symbol);
                    } else {
                        return Err(format!("Input parameter '{}' not provided", input.name));
                    }
                }

                Statement::OutputDeclaration(output) => {
                    let symbol = SymbolInfo {
                        name: output.name.clone(),
                        kind: SymbolKind::Variable,
                        data_type: output.data_type.clone(),
                        initial_value: None,
                        range: None,
                        value: None,
                    };
                    self.symbol_table
                        .insert(output.name.clone(), symbol.clone());
                    self.output_table
                        .insert(output.name.clone(), symbol.clone());
                }

                Statement::VariableDeclaration(var_decl) => {
                    let initial_value = self.evaluate_expression(&var_decl.initial_value)?;
                    let symbol = SymbolInfo {
                        name: var_decl.name.clone(),
                        kind: SymbolKind::Variable,
                        data_type: var_decl.data_type.clone(),
                        initial_value: None,
                        range: None,
                        value: Some(initial_value),
                    };
                    self.symbol_table
                        .insert(var_decl.name.clone(), symbol.clone());
                    self.var_table.insert(var_decl.name.clone(), symbol.clone());
                }
            }
        }
        Ok(())
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
                    Operator::Add => self.apply_op(
                        left_value,
                        right_value,
                        Box::new(|a, b| match (a, b) {
                            (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
                            _ => panic!("Type mismatch in addition"),
                        }),
                    ),
                    Operator::Subtract => self.apply_op(
                        left_value,
                        right_value,
                        Box::new(|a, b| match (a, b) {
                            (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
                            _ => panic!("Type mismatch in subtraction"),
                        }),
                    ),
                    Operator::Multiply => self.apply_op(
                        left_value,
                        right_value,
                        Box::new(|a, b| match (a, b) {
                            (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
                            _ => panic!("Type mismatch in multiplication"),
                        }),
                    ),
                    Operator::Divide => {
                        if let Value::Float(0.0) = right_value {
                            Value::Float(0.0)
                        } else {
                            self.apply_op(
                                left_value,
                                right_value,
                                Box::new(|a, b| match (a, b) {
                                    (Value::Float(x), Value::Float(y)) => Value::Float(x / y),
                                    _ => panic!("Type mismatch in division"),
                                }),
                            )
                        }
                    }
                    Operator::Modulo => {
                        if let Value::Float(0.0) = right_value {
                            Value::Float(0.0)
                        } else {
                            self.apply_op(
                                left_value,
                                right_value,
                                Box::new(|a, b| match (a, b) {
                                    (Value::Float(x), Value::Float(y)) => Value::Float(x % y),
                                    _ => panic!("Type mismatch in modulo operation"),
                                }),
                            )
                        }
                    }
                };

                Ok(evaluated_value)
            }

            Expression::FunctionCall { name, arguments } => {
                let func = match self.function_table.get(name) {
                    Some(func) => func,
                    None => return Err(format!("Function '{}' not found", name)),
                };

                match self.evaluate_function(&func, arguments) {
                    Ok(result) => {
                        let func_info = self
                            .function_table
                            .get(name)
                            .ok_or_else(|| format!("Function '{}' not found", name))?;
                        match func_info.return_type {
                            Type::Float => Ok(Value::Float(result)),
                            Type::Buffer => Ok(Value::Buffer(vec![result])),
                        }
                    }
                    Err(e) => Err(format!("Error evaluating function '{}': {}", name, e)),
                }
            }
        }
    }

    fn apply_op(&self, left: Value, right: Value, op: Box<dyn Fn(Value, Value) -> Value>) -> Value {
        match (left, right) {
            (Value::Float(l), Value::Float(r)) => op(Value::Float(l), Value::Float(r)),
            (Value::Buffer(l), Value::Buffer(r)) => {
                let result: Vec<f32> = l
                    .iter()
                    .zip(r.iter())
                    .map(|(a, b)| {
                        let v = op(Value::Float(*a), Value::Float(*b));
                        match v {
                            Value::Float(val) => val,
                            _ => panic!("Operator applied to non-float values"),
                        }
                    })
                    .collect();
                Value::Buffer(result)
            }
            (Value::Buffer(b), Value::Float(f)) | (Value::Float(f), Value::Buffer(b)) => {
                let result = b
                    .iter()
                    .map(|v| match op(Value::Float(*v), Value::Float(f)) {
                        Value::Float(val) => val,
                        _ => panic!("Operator applied to non-float values"),
                    })
                    .collect();
                Value::Buffer(result)
            }
        }
    }

    fn evaluate_function(
        &self,
        func: &FunctionInfo,
        args: &Vec<Expression>,
    ) -> Result<f32, String> {
        if func.arguments.len() != args.len() {
            return Err(format!(
                "Function '{}' expects {} arguments, but got {}",
                func.name,
                func.arguments.len(),
                args.len()
            ));
        }

        let mut evaluated_args = Vec::new();
        for arg in args {
            let value = self.evaluate_expression(&arg)?;
            match value {
                Value::Float(val) => evaluated_args.push(val),
                Value::Buffer(_) => {
                    return Err(format!(
                        "Function '{}' expects a single float argument, but got a buffer",
                        func.name
                    ));
                }
            }
        }

        match func.name.as_str() {
            "sin" => Ok(evaluated_args[0].sin()),
            "cos" => Ok(evaluated_args[0].cos()),
            "tan" => Ok(evaluated_args[0].tan()),
            "asin" => Ok(evaluated_args[0].asin()),
            "acos" => Ok(evaluated_args[0].acos()),
            "atan" => Ok(evaluated_args[0].atan()),
            "abs" => Ok(evaluated_args[0].abs()),
            "sgn" => Ok(evaluated_args[0].signum()),
            "min" => Ok(evaluated_args[0].min(evaluated_args[1])),
            "max" => Ok(evaluated_args[0].max(evaluated_args[1])),
            "clamp" => Ok(evaluated_args[0].clamp(evaluated_args[1], evaluated_args[2])),
            "pow" => Ok(evaluated_args[0].powf(evaluated_args[1])),
            "sqrt" => Ok(evaluated_args[0].sqrt()),
            "log" => Ok(evaluated_args[0].ln()),
            "log2" => Ok(evaluated_args[0].log2()),
            "log10" => Ok(evaluated_args[0].log10()),
            "saw" => {
                let phase = evaluated_args[0] % 1.0;
                Ok(2.0 * (phase - 0.5))
            }
            "tri" => {
                let phase = evaluated_args[0] % 1.0;
                if phase < 0.5 {
                    Ok(4.0 * phase - 1.0)
                } else {
                    Ok(-4.0 * (phase - 1.0) + 1.0)
                }
            }
            "square" => {
                let phase = evaluated_args[0] % 1.0;
                if phase < 0.5 { Ok(1.0) } else { Ok(-1.0) }
            }
            "rand" => Ok(rand::random::<f32>()),
            "sample" | "sample_rate" | "bpm" | "time" | "phase" | "mix" | "lerp" => {
                Err(format!("Function '{}' is not implemented", func.name))
            }
            "pi" => Ok(std::f32::consts::PI),
            _ => Err(format!("Unknown function '{}'", func.name)),
        }
    }
}
