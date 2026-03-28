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
    ast::Range,
    error::{EK, ErrorCollector, Phase, Pl, Sv},
};
use std::panic::Location;

impl ErrorCollector {
    /// Automatically emits a CompilerBug error with the provided range, phase, and dev message,
    /// combined with the location of the caller.
    #[track_caller]
    pub(crate) fn comp_bug(&mut self, range: Range, phase: Phase, dev_msg: &str) {
        // Get the location of the caller
        let loc = Location::caller();
        // Format the location string
        let loc_str = format!("{}:{}:{}", loc.file(), loc.line(), loc.column());
        // Combine the location and the dev message into one string and emit
        let full = format!("{} -- {}", loc_str, dev_msg);
        self.emit(
            EK::CompilerBug,
            range,
            phase,
            Sv::CompilerBug,
            Pl::Str(full),
        );
    }
}
