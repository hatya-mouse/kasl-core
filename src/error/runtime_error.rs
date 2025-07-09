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

use knodiq_engine::error::TrackError;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum RuntimeError {
    SymbolNotFound {
        name: String,
        line: usize,
    },
    SymbolAlreadyDefined {
        name: String,
        line: usize,
    },
    InputNotProvided {
        name: String,
        line: usize,
    },
    InvalidOperand {
        operator: String,
        operand: String,
        line: usize,
    },
    FunctionInvalidArgumentError {
        name: String,
        line: usize,
    },
    FunctionNotFound {
        name: String,
        line: usize,
    },
    Unknown {
        line: usize,
    },
}

impl RuntimeError {
    pub fn line(&self) -> usize {
        match self {
            RuntimeError::SymbolNotFound { line, .. } => *line,
            RuntimeError::SymbolAlreadyDefined { line, .. } => *line,
            RuntimeError::InputNotProvided { line, .. } => *line,
            RuntimeError::InvalidOperand { line, .. } => *line,
            RuntimeError::FunctionInvalidArgumentError { line, .. } => *line,
            RuntimeError::FunctionNotFound { line, .. } => *line,
            RuntimeError::Unknown { line } => *line,
        }
    }
}

impl TrackError for RuntimeError {
    fn clone_box(&self) -> Box<dyn TrackError> {
        Box::new(self.clone())
    }
}

impl Error for RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Runtime Error: {}",
            match self {
                RuntimeError::SymbolNotFound { name, line } => {
                    format!("Symbol '{}' not found at line {}", name, line)
                }
                RuntimeError::SymbolAlreadyDefined { name, line } => {
                    format!("Symbol '{}' already defined at line {}", name, line)
                }
                RuntimeError::InputNotProvided { name, line } => {
                    format!("Input '{}' not provided at line {}", name, line)
                }
                RuntimeError::InvalidOperand {
                    operator,
                    operand,
                    line,
                } => {
                    format!(
                        "Invalid operand '{}' for operator '{}' at line {}",
                        operand, operator, line
                    )
                }
                RuntimeError::FunctionInvalidArgumentError { name, line } => {
                    format!("Invalid arguments for function '{}' at line {}", name, line)
                }
                RuntimeError::FunctionNotFound { name, line } => {
                    format!("Function '{}' not found at line {}", name, line)
                }
                RuntimeError::Unknown { line } => {
                    format!("Unknown error at line {}", line)
                }
            }
        )
    }
}
