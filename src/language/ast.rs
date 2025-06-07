use crate::{FunctionInfo, SymbolInfo};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug, PartialEq, Clone)]
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
        functions: &HashMap<String, FunctionInfo>,
    ) -> Result<Type, String> {
        match self {
            Expression::BinaryOp { left, right, .. } => {
                let left_type = left.get_expression_type(symbols, functions);
                let right_type = right.get_expression_type(symbols, functions);
                match (left_type, right_type) {
                    (Ok(left_type), Ok(right_type)) => self.combine_types(left_type, right_type),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                }
            }
            Expression::Literal(_) => Ok(Type::Float),
            Expression::Identifier(name) => symbols
                .get(name)
                .map_or(Ok(Type::Float), |info| Ok(info.data_type.clone())),
            Expression::FunctionCall { name, arguments } => {
                functions
                    .get(name)
                    .map_or(Err(format!("Function '{}' not found", name)), |func| {
                        if func.arguments.len() == arguments.len() {
                            Ok(func.return_type.clone())
                        } else {
                            Err(format!(
                                "Function '{}' expects {} arguments, but got {}",
                                name,
                                func.arguments.len(),
                                arguments.len()
                            ))
                        }
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
