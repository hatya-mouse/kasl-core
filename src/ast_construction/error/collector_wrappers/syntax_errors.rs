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
    pub(crate) fn invalid_struct_stmt(&mut self, range: Range, phase: Phase, stmt_kind: String) {
        self.emit(
            EK::InvalidStructStmt,
            range,
            phase,
            Sv::Error,
            Pl::Str(stmt_kind),
        );
    }

    pub(crate) fn top_level_struct_field(&mut self, range: Range, phase: Phase, field_name: &str) {
        self.emit(
            EK::TopLevelStructField,
            range,
            phase,
            Sv::Error,
            Pl::Str(field_name.to_string()),
        );
    }

    pub(crate) fn func_call_in_l_value(&mut self, range: Range, phase: Phase) {
        self.emit(EK::FuncCallInLValue, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn assign_type_mismatch(
        &mut self,
        range: Range,
        phase: Phase,
        target_type: String,
        value_type: String,
    ) {
        self.emit(
            EK::AssignTypeMismatch,
            range,
            phase,
            Sv::Error,
            Pl::StrPair(target_type, value_type),
        );
    }

    pub(crate) fn expr_ends_with_dot(&mut self, range: Range, phase: Phase) {
        self.emit(EK::ExprEndsWithDot, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn non_member_token_after_dot(&mut self, range: Range, phase: Phase) {
        self.emit(
            EK::NonMemberTokenAfterDot,
            range,
            phase,
            Sv::Error,
            Pl::None,
        );
    }

    pub(crate) fn expr_begins_with_dot(&mut self, range: Range, phase: Phase) {
        self.emit(EK::ExprBeginsWithDot, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn invalid_l_value(&mut self, range: Range, phase: Phase) {
        self.emit(EK::InvalidLValue, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn expr_ends_with_type(&mut self, range: Range, phase: Phase) {
        self.emit(EK::ExprEndsWithType, range, phase, Sv::Error, Pl::None);
    }
}
