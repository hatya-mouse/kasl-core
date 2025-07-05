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
pub struct InterpreterError {
    pub message: String,
}

impl TrackError for InterpreterError {
    fn clone_box(&self) -> Box<dyn TrackError> {
        Box::new(self.clone())
    }
}

impl Error for InterpreterError {}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interpreter Error: {}", self.message)
    }
}
