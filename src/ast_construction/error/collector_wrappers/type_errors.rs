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
    pub(crate) fn type_not_found(&mut self, range: Range, phase: Phase, type_name: String) {
        self.emit(
            EK::TypeNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(type_name),
        );
    }

    pub(crate) fn non_bool_type_for_condition(
        &mut self,
        range: Range,
        phase: Phase,
        got_type: String,
    ) {
        self.emit(
            EK::NonBoolTypeForCondition,
            range,
            phase,
            Sv::Error,
            Pl::Str(got_type),
        );
    }

    pub(crate) fn return_type_mismatch(
        &mut self,
        range: Range,
        phase: Phase,
        expected_type: String,
        value_type: String,
    ) {
        self.emit(
            EK::ReturnTypeMismatch,
            range,
            phase,
            Sv::Error,
            Pl::StrPair(expected_type, value_type),
        );
    }

    pub(crate) fn return_value_for_no_return_func(&mut self, range: Range, phase: Phase) {
        self.emit(
            EK::ReturnValueForNoReturnFunc,
            range,
            phase,
            Sv::Error,
            Pl::None,
        );
    }

    pub(crate) fn return_without_value_for_return_func(
        &mut self,
        range: Range,
        phase: Phase,
        return_type: String,
    ) {
        self.emit(
            EK::ReturnWithoutValueForReturnFunc,
            range,
            phase,
            Sv::Error,
            Pl::Str(return_type),
        );
    }

    pub(crate) fn subscript_on_non_array(&mut self, range: Range, phase: Phase, type_name: String) {
        self.emit(
            EK::SubscriptOnNonArray,
            range,
            phase,
            Sv::Error,
            Pl::Str(type_name),
        );
    }

    pub(crate) fn non_integer_in_subscript(
        &mut self,
        range: Range,
        phase: Phase,
        type_name: String,
    ) {
        self.emit(
            EK::NonIntegerInSubscript,
            range,
            phase,
            Sv::Error,
            Pl::Str(type_name),
        );
    }

    pub(crate) fn non_integer_for_count(&mut self, range: Range, phase: Phase, type_name: String) {
        self.emit(
            EK::NonIntegerForCount,
            range,
            phase,
            Sv::Error,
            Pl::Str(type_name),
        );
    }

    pub(crate) fn array_item_type_mismatch(
        &mut self,
        range: Range,
        phase: Phase,
        first_type: String,
        item_type: String,
    ) {
        self.emit(
            EK::ArrayItemTypeMismatch,
            range,
            phase,
            Sv::Error,
            Pl::StrPair(first_type, item_type),
        );
    }
}
