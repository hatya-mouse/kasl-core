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

use crate::SymbolPath;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConstructorErrorType {
    ConsecutiveDots,
    TrailingDot,
    TypeNotFound(String),
    ExpectType,
    Invalid {
        scope: ScopeType,
        cause: StatementType,
    },
    InvalidRequiredBy,
    AmbiguousDeclaration(String),
    NotEnoughParamForOp(OperatorKind),
    DependencyCycle(SymbolPath),

    Placeholder,
}

impl ConstructorErrorType {
    pub fn format(&self) -> String {
        match self {
            ConstructorErrorType::ConsecutiveDots => {
                "Consecutive dots are not allowed here.".to_string()
            }
            ConstructorErrorType::TrailingDot => "Trailing dot is not allowed here.".to_string(),
            ConstructorErrorType::TypeNotFound(type_name) => {
                format!("Type '{}' not found here.", type_name)
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
            ConstructorErrorType::NotEnoughParamForOp(op_type) => {
                format!(
                    "Not enough number of parameters for {} operator.",
                    op_type.to_string()
                )
            }
            ConstructorErrorType::DependencyCycle(symbol_path) => {
                format!(
                    "Cannot infer the type of '{}' due to a dependency cycle.",
                    symbol_path
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
    Postfix,
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
            StatementType::Postfix => "postfix operator".to_string(),
            StatementType::Block => "block statement".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OperatorKind {
    Infix,
    Prefix,
    Postfix,
}

impl OperatorKind {
    pub fn to_string(&self) -> String {
        match self {
            OperatorKind::Infix => "infix".to_string(),
            OperatorKind::Prefix => "prefix".to_string(),
            OperatorKind::Postfix => "postfix".to_string(),
        }
    }
}
