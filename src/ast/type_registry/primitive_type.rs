//
// © 2025-2026 Shuntaro Kasatani
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

use cranelift::prelude::{Type, types};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Int,
    Float,
    Bool,
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveType::Int => write!(f, "Int"),
            PrimitiveType::Float => write!(f, "Float"),
            PrimitiveType::Bool => write!(f, "Bool"),
        }
    }
}

impl PrimitiveType {
    pub fn as_ir_type(&self) -> Type {
        match self {
            PrimitiveType::Int => types::I32,
            PrimitiveType::Float => types::F32,
            PrimitiveType::Bool => types::I8,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            PrimitiveType::Bool => 1,
            PrimitiveType::Int => 4,
            PrimitiveType::Float => 4,
        }
    }

    pub fn alignment(&self) -> usize {
        match self {
            PrimitiveType::Bool => 1,
            PrimitiveType::Int => 4,
            PrimitiveType::Float => 4,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Int" => Some(PrimitiveType::Int),
            "Float" => Some(PrimitiveType::Float),
            "Bool" => Some(PrimitiveType::Bool),
            _ => None,
        }
    }
}
