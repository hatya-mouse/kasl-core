//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::ast_nodes::{Expr, VariableID, type_registry::ResolvedType};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct LValue {
    pub kind: LValueKind,
    pub value_type: ResolvedType,
}

impl LValue {
    pub fn new(kind: LValueKind, value_type: ResolvedType) -> Self {
        Self { kind, value_type }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum LValueKind {
    Identifier(VariableID),
    StructField { lhs: Box<LValue>, offset: i32 },
    Subscript { lhs: Box<LValue>, index: Box<Expr> },
}
