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

use std::fmt::Display;

/// An ID used to identify a variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct VariableID(usize);

impl VariableID {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

impl Display for VariableID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A symbol ID used in RawSymbolTable. Not related to VariableID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ParserStmtID(usize);

impl ParserStmtID {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

/// An ID used to identify a struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct StructID(usize);

impl StructID {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

impl Display for StructID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An ID used to identify a function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct FunctionID(usize);

impl FunctionID {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

impl Display for FunctionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An ID used to identify an operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct OperatorID(usize);

impl OperatorID {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

impl Display for OperatorID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An ID used to identify a namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct NameSpaceID(usize);

impl NameSpaceID {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

impl Display for NameSpaceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
