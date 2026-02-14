//
// Copyright 2025-2026 Shuntaro Kasatani
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

use crate::{Range, error::CanonicalMeta};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct ErrorRecord {
    pub key: ErrorKey,
    pub earliest_phase: Phase,
    pub ranges: HashSet<Range>,
    pub severity: Severity,
}

impl ErrorRecord {
    pub fn new(
        kind: ErrorKind,
        range: Range,
        phase: Phase,
        severity: Severity,
        meta: CanonicalMeta,
    ) -> Self {
        ErrorRecord {
            key: ErrorKey::new(kind, meta),
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
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ErrorKey {
    pub kind: ErrorKind,
    pub meta: CanonicalMeta,
}

impl ErrorKey {
    pub fn new(kind: ErrorKind, meta: CanonicalMeta) -> Self {
        ErrorKey { kind, meta }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    Parse = 0,
    SymbolTableConstruction = 10,
    Validation = 20,
    TypeCollection = 30,
    TopLevelCollection = 40,
    MemberCollection = 50,
    TypeResolution = 60,
}

#[derive(Clone, Debug)]
pub struct Severity {}
