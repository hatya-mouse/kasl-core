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
    pub(crate) fn global_func_cannot_be_static(
        &mut self,
        range: Range,
        phase: Phase,
        func_name: &str,
    ) {
        self.emit(
            EK::GlobalFuncCannotBeStatic,
            range,
            phase,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub(crate) fn func_not_found(&mut self, range: Range, phase: Phase, func_name: &str) {
        self.emit(
            EK::FuncNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub(crate) fn no_return_func_in_expr(&mut self, range: Range, phase: Phase, func_name: &str) {
        self.emit(
            EK::NoReturnFuncInExpr,
            range,
            phase,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub(crate) fn arg_order_incorrect(&mut self, range: Range, phase: Phase, param_label: &str) {
        self.emit(
            EK::ArgOrderIncorrect,
            range,
            phase,
            Sv::Error,
            Pl::Str(param_label.to_string()),
        );
    }

    pub(crate) fn duplicate_arg(&mut self, range: Range, phase: Phase, param_label: &str) {
        self.emit(
            EK::DuplicateArg,
            range,
            phase,
            Sv::Error,
            Pl::Str(param_label.to_string()),
        );
    }

    pub(crate) fn extra_arg(&mut self, range: Range, phase: Phase) {
        self.emit(EK::ExtraArg, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn missing_arg(&mut self, range: Range, phase: Phase) {
        self.emit(EK::MissingArg, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn missing_arg_label(&mut self, range: Range, phase: Phase) {
        self.emit(EK::MissingArgLabel, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn arg_type_mismatch(&mut self, range: Range, phase: Phase, arg_name: &str) {
        self.emit(
            EK::ArgTypeMismatch,
            range,
            phase,
            Sv::Error,
            Pl::Str(arg_name.to_string()),
        );
    }

    pub(crate) fn duplicate_func_name(&mut self, range: Range, phase: Phase, func_name: &str) {
        self.emit(
            EK::DuplicateFuncName,
            range,
            phase,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub(crate) fn recursive_call(&mut self, range: Range, phase: Phase) {
        self.emit(EK::RecursiveCall, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn missing_return(&mut self, range: Range, phase: Phase) {
        self.emit(EK::MissingReturn, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn static_func_call_on_instance(
        &mut self,
        range: Range,
        phase: Phase,
        func_name: &str,
    ) {
        self.emit(
            EK::StaticFuncCallOnInstance,
            range,
            phase,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub(crate) fn static_call_of_instance_func(
        &mut self,
        range: Range,
        phase: Phase,
        func_name: &str,
    ) {
        self.emit(
            EK::StaticCallOfInstanceFunc,
            range,
            phase,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub(crate) fn builtin_func_not_found(&mut self, range: Range, phase: Phase, func_name: &str) {
        self.emit(
            EK::BuiltinFuncNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub(crate) fn builtin_arg_type_mismatch(&mut self, range: Range, phase: Phase) {
        self.emit(
            EK::BuiltinArgTypeMismatch,
            range,
            phase,
            Sv::Error,
            Pl::None,
        );
    }
}
