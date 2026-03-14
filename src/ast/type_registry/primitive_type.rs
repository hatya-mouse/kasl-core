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

use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum PrimitiveType {
    Int,
    Float,
    Bool,
    Void,
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveType::Int => write!(f, "Int"),
            PrimitiveType::Float => write!(f, "Float"),
            PrimitiveType::Bool => write!(f, "Bool"),
            PrimitiveType::Void => write!(f, "Void"),
        }
    }
}

impl PrimitiveType {
    pub fn size(&self) -> u32 {
        match self {
            PrimitiveType::Bool => 1,
            PrimitiveType::Int => 4,
            PrimitiveType::Float => 4,
            PrimitiveType::Void => 0,
        }
    }

    pub fn alignment(&self) -> u8 {
        match self {
            PrimitiveType::Bool => 1,
            PrimitiveType::Int => 4,
            PrimitiveType::Float => 4,
            PrimitiveType::Void => 1,
        }
    }
}

impl FromStr for PrimitiveType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Int" => Ok(PrimitiveType::Int),
            "Float" => Ok(PrimitiveType::Float),
            "Bool" => Ok(PrimitiveType::Bool),
            _ => Err(()),
        }
    }
}
