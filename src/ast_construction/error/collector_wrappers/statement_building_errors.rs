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
    /// Wrapper function for ParamNotFound error.
    pub fn param_not_found(
        &mut self,
        range: Range,
        phase: Phase,
        func_path: &str,
        param_label: &str,
    ) {
        self.emit(
            EK::ParamNotFound,
            range,
            phase,
            Sv::Error,
            Pl::StrPair(func_path.to_string(), param_label.to_string()),
        );
    }

    /// Wrapper function for TooManyParams error.
    pub fn too_many_params(
        &mut self,
        range: Range,
        phase: Phase,
        func_path: &str,
        expected_num: usize,
        actual_num: usize,
    ) {
        self.emit(
            EK::TooManyParams,
            range,
            phase,
            Sv::Error,
            Pl::StrAndNumPair(func_path.to_string(), expected_num, actual_num),
        );
    }

    /// Wrapper function for NotEnoughParams error.
    pub fn not_enough_params(
        &mut self,
        range: Range,
        phase: Phase,
        func_path: &str,
        required_num: usize,
        actual_num: usize,
    ) {
        self.emit(
            EK::NotEnoughParams,
            range,
            phase,
            Sv::Error,
            Pl::StrAndNumPair(func_path.to_string(), required_num, actual_num),
        );
    }

    /// Wrapper function for RecursiveFunc error.
    pub fn recursive_func(&mut self, range: Range, phase: Phase, func_path: &str) {
        self.emit(
            EK::RecursiveFunc,
            range,
            phase,
            Sv::Error,
            Pl::Str(func_path.to_string()),
        );
    }
}
