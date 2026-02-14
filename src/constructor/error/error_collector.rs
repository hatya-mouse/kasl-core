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

use std::collections::HashMap;

use crate::{
    Range,
    error::{CanonicalMeta, ErrorKey, ErrorKind, ErrorRecord, Payload, Phase, Severity},
};

pub struct ErrorCollector {
    records: HashMap<ErrorKey, ErrorRecord>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        ErrorCollector {
            records: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        kind: ErrorKind,
        range: Range,
        phase: Phase,
        severity: Severity,
        payload: Payload,
    ) {
        // 1. Convert payload to canonical meta
        let meta = self.canonicalize(&payload);

        // 2. Generate a key
        let key = ErrorKey::new(kind, meta);

        // 3. Register / Update the error record
        if let Some(record) = self.records.get_mut(&key) {
            // Prefer the record from the earlier phase
            if phase < record.earliest_phase {
                record.earliest_phase = phase;
            }
            record.add_range(range);
        } else {
            let new_record = ErrorRecord::new(kind, range, phase, severity, meta);
            self.records.insert(key, new_record);
        }
    }

    pub fn canonicalize(&self, payload: &Payload) -> CanonicalMeta {
        match payload {
            Payload::None => CanonicalMeta::None,
        }
    }
}
