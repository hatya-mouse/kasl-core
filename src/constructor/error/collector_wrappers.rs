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

use std::panic::Location;

use crate::{
    LiteralBind, Range,
    error::{EK, ErrorCollector, Phase, Pl, Sv},
};

impl ErrorCollector {
    pub fn dup_sym(&mut self, range: Range, phase: Phase, sym: &str) {
        self.emit(
            EK::DuplicateSymbol,
            range,
            phase,
            Sv::Error,
            Pl::Str(sym.to_string()),
        );
    }

    pub fn req_by_outside_type(&mut self, range: Range, phase: Phase) {
        self.emit(EK::RequiredByOutsideType, range, phase, Sv::Error, Pl::None);
    }

    pub fn invalid_param_numbers_for_infix(
        &mut self,
        range: Range,
        phase: Phase,
        got_params: usize,
    ) {
        self.emit(
            EK::InvalidParamNumbersForInfix,
            range,
            phase,
            Sv::Error,
            Pl::Num(got_params),
        );
    }

    pub fn invalid_param_numbers_for_prefix(
        &mut self,
        range: Range,
        phase: Phase,
        got_params: usize,
    ) {
        self.emit(
            EK::InvalidParamNumbersForPrefix,
            range,
            phase,
            Sv::Error,
            Pl::Num(got_params),
        );
    }

    pub fn dup_literal_bind(&mut self, range: Range, phase: Phase, bind: LiteralBind) {
        self.emit(
            EK::DuplicateLiteralBind,
            range,
            phase,
            Sv::Error,
            Pl::Str(bind.to_string()),
        );
    }

    #[track_caller]
    pub fn comp_bug(&mut self, range: Range, phase: Phase, dev_msg: &str) {
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
