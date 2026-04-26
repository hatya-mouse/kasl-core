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
    ast_nodes::Range,
    error::{EK, ErrorCollector, Phase, Pl, Sv},
};

impl ErrorCollector {
    pub(crate) fn member_access_on_primitive(
        &mut self,
        range: Range,
        phase: Phase,
        type_name: String,
    ) {
        self.emit(
            EK::MemberAccessOnPrimitive,
            range,
            phase,
            Sv::Error,
            Pl::StrVec(vec![type_name]),
        );
    }

    pub(crate) fn member_access_on_array(&mut self, range: Range, phase: Phase, type_name: String) {
        self.emit(
            EK::MemberAccessOnArray,
            range,
            phase,
            Sv::Error,
            Pl::StrVec(vec![type_name]),
        );
    }

    pub(crate) fn member_field_not_found(
        &mut self,
        range: Range,
        phase: Phase,
        struct_name: &str,
        field_name: &str,
    ) {
        self.emit(
            EK::MemberFieldNotFound,
            range,
            phase,
            Sv::Error,
            Pl::StrVec(vec![struct_name.to_string(), field_name.to_string()]),
        );
    }

    pub(crate) fn member_func_not_found(
        &mut self,
        range: Range,
        phase: Phase,
        struct_name: &str,
        func_name: &str,
    ) {
        self.emit(
            EK::MemberFuncNotFound,
            range,
            phase,
            Sv::Error,
            Pl::StrVec(vec![struct_name.to_string(), func_name.to_string()]),
        );
    }

    pub(crate) fn struct_cycle(&mut self, range: Range, phase: Phase, struct_name: &str) {
        self.emit(
            EK::StructCycle,
            range,
            phase,
            Sv::Error,
            Pl::StrVec(vec![struct_name.to_string()]),
        )
    }

    pub(crate) fn arg_for_struct_init(&mut self, range: Range, phase: Phase) {
        self.emit(EK::ArgForStructInit, range, phase, Sv::Error, Pl::None);
    }
}
