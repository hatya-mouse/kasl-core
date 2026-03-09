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
    pub fn no_type_annotation_or_def_val(&mut self, range: Range, phase: Phase) {
        self.emit(
            EK::NoTypeAnnotationOrDefVal,
            range,
            phase,
            Sv::Error,
            Pl::None,
        );
    }

    pub fn var_not_found(&mut self, range: Range, phase: Phase, var_name: &str) {
        self.emit(
            EK::VarNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(var_name.to_string()),
        );
    }

    pub fn type_annotation_mismatch(
        &mut self,
        range: Range,
        phase: Phase,
        annotation_type: String,
        expr_type: String,
    ) {
        self.emit(
            EK::TypeAnnotationMismatch,
            range,
            phase,
            Sv::Error,
            Pl::StrPair(annotation_type.to_string(), expr_type.to_string()),
        );
    }

    pub fn duplicate_var_name(&mut self, range: Range, phase: Phase, var_name: &str) {
        self.emit(
            EK::DuplicateVarName,
            range,
            phase,
            Sv::Error,
            Pl::Str(var_name.to_string()),
        );
    }
}
