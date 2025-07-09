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

use knodiq_engine::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
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
    ForLoop(ForLoopStatement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputDeclarationStatement {
    pub name: String,
    pub data_type: Type,
    pub initial_value: Option<Expression>,
    pub range: Option<(f32, f32)>,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutputDeclarationStatement {
    pub name: String,
    pub data_type: Type,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarationStatement {
    pub name: String,
    pub initial_value: Expression,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentStatement {
    pub target_name: String,
    pub value: Expression,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForLoopStatement {
    pub variable_name: String,
    pub iterable: Expression,
    pub body: Vec<Statement>,
    pub line: usize,
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
