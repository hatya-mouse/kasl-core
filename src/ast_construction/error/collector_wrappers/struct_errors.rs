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
    pub fn member_access_on_primitive(&mut self, range: Range, phase: Phase) {
        self.emit(
            EK::MemberAccessOnPrimitive,
            range,
            phase,
            Sv::Error,
            Pl::None,
        );
    }

    pub fn member_field_not_found(
        &mut self,
        range: Range,
        phase: Phase,
        struct_name: String,
        field_name: String,
    ) {
        self.emit(
            EK::MemberFieldNotFound,
            range,
            phase,
            Sv::Error,
            Pl::StrPair(struct_name, field_name),
        );
    }

    pub fn member_func_not_found(
        &mut self,
        range: Range,
        phase: Phase,
        struct_name: String,
        func_name: String,
    ) {
        self.emit(
            EK::MemberFuncNotFound,
            range,
            phase,
            Sv::Error,
            Pl::StrPair(struct_name, func_name),
        );
    }

    pub fn duplicate_struct_name(&mut self, range: Range, phase: Phase, struct_name: &str) {
        self.emit(
            EK::DuplicateStructName,
            range,
            phase,
            Sv::Error,
            Pl::Str(struct_name.to_string()),
        );
    }
}
