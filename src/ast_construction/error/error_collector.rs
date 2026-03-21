use crate::{
    Range,
    error::{ErrorKey, ErrorKind, ErrorRecord, Payload, Phase, Severity},
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ErrorCollector {
    pub records: HashMap<ErrorKey, ErrorRecord>,
}

impl Default for ErrorCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorCollector {
    pub fn new() -> Self {
        ErrorCollector {
            records: HashMap::new(),
        }
    }

    pub fn push_error(&mut self, error: ErrorRecord) {
        let key = error.key.clone();
        if let Some(record) = self.records.get_mut(&key) {
            // Prefer the record from the earlier phase
            if error.earliest_phase < record.earliest_phase {
                record.earliest_phase = error.earliest_phase;
            }
            record.extend_range(error.ranges);
        } else {
            self.records.insert(key, error);
        }
    }

    pub fn emit(
        &mut self,
        kind: ErrorKind,
        range: Range,
        phase: Phase,
        severity: Severity,
        payload: Payload,
    ) {
        // 1. Generate a key
        let key = ErrorKey::new(kind, payload);

        // 2. Register / Update the error record
        if let Some(record) = self.records.get_mut(&key) {
            // Prefer the record from the earlier phase
            if phase < record.earliest_phase {
                record.earliest_phase = phase;
            }
            record.add_range(range);
        } else {
            let new_record = ErrorRecord::new(key.clone(), range, phase, severity);
            self.records.insert(key, new_record);
        }
    }

    pub fn as_result(&self) -> Result<(), Vec<ErrorRecord>> {
        if self.has_error() {
            Err(self.records.values().cloned().collect())
        } else {
            Ok(())
        }
    }

    pub fn has_error(&self) -> bool {
        !self.records.is_empty()
    }

    pub fn has_error_kind(&self, kind: ErrorKind, payload: Payload) -> bool {
        self.records
            .keys()
            .any(|key| key.kind == kind && key.payload == payload)
    }
}
