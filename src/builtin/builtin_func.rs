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

use crate::ast::type_registry::ResolvedType;
use kasl_ir::ir::{IRBuilder, Value};
use std::fmt::Display;

pub type BuiltinFuncTranslator = Box<dyn Fn(&mut IRBuilder, &[Value]) -> Value>;

pub struct BuiltinFunc {
    pub name: &'static str,
    pub params: Vec<ResolvedType>,
    pub return_type: ResolvedType,
    pub translator: BuiltinFuncTranslator,
}

/// An ID used to identify an builtin function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct BuiltinFuncID(usize);

impl BuiltinFuncID {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

impl Display for BuiltinFuncID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
