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

use crate::{
    ast_nodes::Range,
    error::{ErrorKind, Payload},
};
use std::collections::HashSet;

/// A record of an error that occurred during compilation. The same error will automatically be merged into one record.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ErrorRecord {
    pub key: ErrorKey,
    pub earliest_phase: Phase,
    pub ranges: HashSet<Range>,
    pub severity: Severity,
}

impl ErrorRecord {
    pub fn new(key: ErrorKey, range: Range, phase: Phase, severity: Severity) -> Self {
        ErrorRecord {
            key,
            earliest_phase: phase,
            ranges: HashSet::from([range]),
            severity,
        }
    }

    pub fn add_range(&mut self, range: Range) {
        if !self.ranges.contains(&range) {
            self.ranges.insert(range);
        }
    }

    pub fn extend_range(&mut self, ranges: HashSet<Range>) {
        for r in ranges {
            self.add_range(r);
        }
    }
}

impl std::fmt::Display for ErrorRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?}] with payload {:?}",
            self.key.kind, self.key.payload
        )
    }
}

impl std::error::Error for ErrorRecord {}

/// A key for an error. Errors with the same key will be merged into one record.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct ErrorKey {
    pub kind: ErrorKind,
    pub payload: Payload,
}

impl ErrorKey {
    pub fn new(kind: ErrorKind, payload: Payload) -> Self {
        ErrorKey { kind, payload }
    }
}

/// Represents the phase of the compilation when an error occurred.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub enum Phase {
    Parse = 0,
    StructCollection = 1,
    GlobalDeclCollection = 2,
    StructGraphAnalyzing = 3,
    StatementBuilding = 4,
    ExprEngine = 5,
    FlowGraphAnalyzing = 6,
    ScopeGraphAnalyzing = 7,
    Backend = 8,
}

/// The severity of the error.
#[derive(Clone, Debug, serde::Serialize)]
pub enum Severity {
    CompilerBug,
    Error,
    Warning,
}
