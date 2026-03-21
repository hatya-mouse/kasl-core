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

use crate::{
    Range,
    error::{ErrorKind, Payload},
};
use std::collections::HashSet;

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

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub enum Phase {
    Parse = 0,
    StructCollection = 1,
    GlobalDeclCollection = 2,
    StructGraphAnalyzing = 3,
    StatementBuilding = 4,
    ExprEngine = 5,
    ScopeGraphAnalyzing = 6,
    Backend = 7,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum Severity {
    CompilerBug,
    Error,
    Warning,
    Info,
}
