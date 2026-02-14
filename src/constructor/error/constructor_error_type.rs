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

use crate::{LiteralBind, SymbolPath};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConstructorErrorType {
    ConsecutiveDots,
    TrailingDot,
    InvalidOperatorParams(String),
    SymbolNotFound(Option<SymbolPath>),
    DuplicateSymbol(String),
    OperatorNotFound(String),
    OperatorCannotBeChained(String),
    ExpectType,
    Invalid {
        scope: ScopeType,
        cause: StatementType,
    },
    InvalidRequiredBy,
    AmbiguousDeclaration(String),
    InvalidParamForOp,
    DependencyCycle(SymbolPath),
    CannotInferType(SymbolPath),
    DuplicateLiteralBind(LiteralBind),
    MissingLiteralBind(LiteralBind),
    NoReturnFunctionInExpr(String),
    UnmatchedParentheses,
    InvalidInfixProperty(String),
    ArityMismatch(String, usize),
    ExprSyntaxError,
    ArgumentNotFound(Option<String>),
    TypeMismatch(SymbolPath, SymbolPath),

    CompilerBug(String),
    Placeholder,
}

impl ConstructorErrorType {
    pub fn format(&self) -> String {
        match self {
            ConstructorErrorType::ConsecutiveDots => {
                "Consecutive dots are not allowed here.".to_string()
            }
            ConstructorErrorType::TrailingDot => "Trailing dot is not allowed here.".to_string(),
            ConstructorErrorType::InvalidOperatorParams(operator_symbol) => {
                format!(
                    "Operator '{}' has wrong number of parameters.",
                    operator_symbol,
                )
            }
            ConstructorErrorType::SymbolNotFound(symbol_path) => match symbol_path {
                Some(path) => format!("Symbol '{}' not found here.", path),
                None => "Symbol not found here.".to_string(),
            },
            ConstructorErrorType::OperatorNotFound(operator_symbol) => {
                format!("Operator '{}' not found here.", operator_symbol)
            }
            ConstructorErrorType::OperatorCannotBeChained(operator_symbol) => {
                format!("Infix operator '{}' cannot be chained.", operator_symbol)
            }
            ConstructorErrorType::DuplicateSymbol(symbol) => {
                format!("Symbol '{}' is already defined.", symbol)
            }
            ConstructorErrorType::ExpectType => "Type name is expected.".to_string(),
            ConstructorErrorType::Invalid { scope, cause } => {
                let cause_str = cause.to_string();
                let capitalized_cause = format!(
                    "{}{}",
                    cause_str.chars().next().unwrap().to_uppercase(),
                    &cause_str[1..]
                );
                format!(
                    "{} is not allowed in {} scope.",
                    capitalized_cause,
                    scope.to_string()
                )
            }
            ConstructorErrorType::InvalidRequiredBy => {
                "Required type name can only be used within structs and protocols with inherits"
                    .to_string()
            }
            ConstructorErrorType::AmbiguousDeclaration(type_name) => {
                format!(
                    "The type of '{}' is unclear. Please add a type annotation (e.g. '{}: Int') or provide a default value (e.g. '{} = 0') so the compiler can know its type.",
                    type_name, type_name, type_name
                )
            }
            ConstructorErrorType::InvalidParamForOp => {
                format!("Invalid parameter for operator.")
            }
            ConstructorErrorType::DependencyCycle(symbol_path) => {
                format!(
                    "Cannot infer the type of '{}' due to a dependency cycle.",
                    symbol_path
                )
            }
            ConstructorErrorType::CannotInferType(symbol_path) => {
                format!("Cannot infer the type of '{}'.", symbol_path)
            }
            ConstructorErrorType::DuplicateLiteralBind(type_bind) => {
                format!(
                    "Duplicate {} initializer.",
                    match type_bind {
                        LiteralBind::BoolLiteral => "bool_literal",
                        LiteralBind::IntLiteral => "int_literal",
                        LiteralBind::FloatLiteral => "float_literal",
                    }
                )
            }
            ConstructorErrorType::MissingLiteralBind(type_bind) => {
                format!(
                    "{} initializer is not declared in the scope despite the literal is used.",
                    match type_bind {
                        LiteralBind::BoolLiteral => "bool_literal",
                        LiteralBind::IntLiteral => "int_literal",
                        LiteralBind::FloatLiteral => "float_literal",
                    }
                )
            }
            ConstructorErrorType::NoReturnFunctionInExpr(name) => {
                format!("This function '{}' does not have a return type.", name)
            }
            ConstructorErrorType::UnmatchedParentheses => {
                format!("Unmatched parentheses.")
            }
            ConstructorErrorType::InvalidInfixProperty(attribute) => {
                format!("Infix property '{}' doesn't exist", attribute)
            }
            ConstructorErrorType::ArityMismatch(function_name, expected) => {
                format!(
                    "Operator '{}' expects {} arguments",
                    function_name, expected
                )
            }
            ConstructorErrorType::ArgumentNotFound(argument_name) => {
                format!(
                    "Argument '{}' not found",
                    argument_name.clone().unwrap_or("unknown".to_string())
                )
            }
            ConstructorErrorType::ExprSyntaxError => {
                format!("Syntax error in expression.")
            }
            ConstructorErrorType::TypeMismatch(value_type, annotation_type) => {
                format!(
                    "Default value is a value of the type '{}', but the type annotation is '{}'",
                    value_type, annotation_type
                )
            }
            ConstructorErrorType::CompilerBug(message) => {
                format!(
                    "Compiler bug: \"{}\" Please report to the developer.",
                    message
                )
            }
            ConstructorErrorType::Placeholder => "PLACEHOLDER ERROR".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScopeType {
    TopLevel,
    Function,
    Struct,
    Protocol,
}

impl ScopeType {
    pub fn to_string(&self) -> String {
        match self {
            ScopeType::TopLevel => "top-level".to_string(),
            ScopeType::Function => "function".to_string(),
            ScopeType::Struct => "struct".to_string(),
            ScopeType::Protocol => "protocol".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StatementType {
    FuncDecl,
    Return,
    Input,
    Output,
    Var,
    State,
    Assign,
    FuncCall,
    If,
    IfElse,
    StructDecl,
    ProtocolDecl,
    Init,
    Infix,
    Prefix,
    Block,
}

impl StatementType {
    pub fn to_string(&self) -> String {
        match self {
            StatementType::FuncDecl => "function declaration".to_string(),
            StatementType::Return => "return statement".to_string(),
            StatementType::Input => "input declaration".to_string(),
            StatementType::Output => "output declaration".to_string(),
            StatementType::Var => "variable declaration".to_string(),
            StatementType::State => "state declaration".to_string(),
            StatementType::Assign => "assignment".to_string(),
            StatementType::FuncCall => "function call".to_string(),
            StatementType::If => "if statement".to_string(),
            StatementType::IfElse => "if-else statement".to_string(),
            StatementType::StructDecl => "struct declaration".to_string(),
            StatementType::ProtocolDecl => "protocol declaration".to_string(),
            StatementType::Init => "initializer".to_string(),
            StatementType::Infix => "infix operator".to_string(),
            StatementType::Prefix => "prefix operator".to_string(),
            StatementType::Block => "block statement".to_string(),
        }
    }
}
