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
    error::{EK, ErrorCollector, Phase, Pl, Sv},
};

impl ErrorCollector {
    pub fn prefix_op_not_found(&mut self, range: Range, phase: Phase, symbol: &str) {
        self.emit(
            EK::PrefixOpNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(symbol.to_string()),
        );
    }

    pub fn infix_or_postfix_op_not_found(&mut self, range: Range, phase: Phase, symbol: &str) {
        self.emit(
            EK::InfixOrPostfixOpNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(symbol.to_string()),
        );
    }

    pub fn op_not_associative(&mut self, range: Range, phase: Phase, symbol: &str) {
        self.emit(
            EK::OpNotAssociative,
            range,
            phase,
            Sv::Error,
            Pl::Str(symbol.to_string()),
        );
    }

    pub fn wrong_param_count_for_infix(
        &mut self,
        range: Range,
        phase: Phase,
        op_symbol: &str,
        param_count: usize,
    ) {
        self.emit(
            EK::WrongParamCountForInfix,
            range,
            phase,
            Sv::Error,
            Pl::StrAndNum(op_symbol.to_string(), param_count),
        );
    }

    pub fn wrong_param_count_for_prefix(
        &mut self,
        range: Range,
        phase: Phase,
        op_symbol: &str,
        param_count: usize,
    ) {
        self.emit(
            EK::WrongParamCountForPrefix,
            range,
            phase,
            Sv::Error,
            Pl::StrAndNum(op_symbol.to_string(), param_count),
        );
    }

    pub fn wrong_param_count_for_postfix(
        &mut self,
        range: Range,
        phase: Phase,
        op_symbol: &str,
        param_count: usize,
    ) {
        self.emit(
            EK::WrongParamCountForPostfix,
            range,
            phase,
            Sv::Error,
            Pl::StrAndNum(op_symbol.to_string(), param_count),
        );
    }
}
