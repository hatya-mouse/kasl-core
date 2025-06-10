use crate::{Function, SymbolInfo, builtin_function::ArgumentTypeSpec};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Float,
    Buffer,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    InputDeclaration(InputDeclarationStatement),
    OutputDeclaration(OutputDeclarationStatement),
    VariableDeclaration(VariableDeclarationStatement),
    Assignment(AssignmentStatement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputDeclarationStatement {
    pub name: String,
    pub data_type: Type,
    pub initial_value: Option<Expression>,
    pub range: Option<(f32, f32)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutputDeclarationStatement {
    pub name: String,
    pub data_type: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarationStatement {
    pub name: String,
    pub data_type: Type,
    pub initial_value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentStatement {
    pub target_name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BinaryOp {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Literal(f32),
    Identifier(String),
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

impl Expression {
    pub fn get_expression_type(
        &self,
        symbols: &HashMap<String, SymbolInfo>,
        functions: &HashMap<String, Function>,
        expected: Option<Type>,
    ) -> Result<Type, String> {
        match self {
            Expression::BinaryOp { left, right, .. } => {
                let left_type = left.get_expression_type(symbols, functions, expected);
                let right_type = right.get_expression_type(symbols, functions, expected);
                match (left_type, right_type) {
                    (Ok(left_type), Ok(right_type)) => self.combine_types(left_type, right_type),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                }
            }
            Expression::Literal(_) => Ok(Type::Float),
            Expression::Identifier(name) => symbols
                .get(name)
                .map_or(Err(format!("Identifier '{}' not found", name)), |info| Ok(info.data_type.clone())),
            Expression::FunctionCall { name, arguments } => {
                functions
                    .get(name)
                    .map_or(Err(format!("Function '{}' not found", name)), |func| {
                        if func.argument_specs.len() != arguments.len() {
                            return Err(format!(
                                "Function '{}' expects {} arguments, found {}",
                                name,
                                func.argument_specs.len(),
                                arguments.len()
                            ));
                        }

                        let mut combined_type: Option<Type> = None;
                        for (arg, spec) in arguments.iter().zip(&func.argument_specs) {
                            let arg_type = arg.get_expression_type(symbols, functions, expected)?;
                            match &spec {
                                ArgumentTypeSpec::Concrete(arg_type_spec) => {
                                    if combined_type.is_none() {
                                        combined_type = Some(*arg_type_spec);
                                    } else {
                                        combined_type = Some(self.combine_types(
                                            combined_type.unwrap(),
                                            *arg_type_spec,
                                        )?);
                                    }
                                }
                                ArgumentTypeSpec::Polymorphic(types) => {
                                    if expected.is_some() && types.contains(&expected.unwrap()) {
                                        if combined_type.is_none() {
                                            combined_type = Some(expected.unwrap());
                                        } else {
                                            combined_type = Some(
                                                self.combine_types(combined_type.unwrap(), expected.unwrap())?,
                                            );
                                        }
                                    } else if types.contains(&arg_type) {
                                        if combined_type.is_none() {
                                            combined_type = Some(arg_type);
                                        } else {
                                            combined_type = Some(
                                                self.combine_types(combined_type.unwrap(), arg_type)?,
                                            );
                                        }
                                    } else {
                                        return Err(format!(
                                            "Argument type {:?} does not match expected types {:?} for function '{}'",
                                            arg_type, types, name
                                        ));
                                    }
                                }
                            }
                        }
                        Ok(combined_type.unwrap_or(Type::Float))
                    })
            }
        }
    }

    fn combine_types(&self, left_type: Type, right_type: Type) -> Result<Type, String> {
        match (left_type, right_type) {
            (Type::Float, Type::Float) => Ok(Type::Float),
            (Type::Buffer, Type::Buffer) => Ok(Type::Buffer),
            (Type::Float, Type::Buffer) | (Type::Buffer, Type::Float) => Ok(Type::Buffer),
        }
    }
}
