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
    /// Wrapper function for VariableNotFound error.
    pub fn var_not_found(&mut self, range: Range, phase: Phase, path: &str) {
        self.emit(
            EK::VariableNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(path.to_string()),
        );
    }

    /// Wrapper function for FunctionNotFound error.
    pub fn func_not_found(&mut self, range: Range, phase: Phase, path: &str) {
        self.emit(
            EK::FunctionNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(path.to_string()),
        );
    }

    /// Wrapper function for OperatorNotFound error.
    pub fn operator_not_found(&mut self, range: Range, phase: Phase, operator: &str) {
        self.emit(
            EK::OperatorNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(operator.to_string()),
        );
    }

    /// Wrapper function for TypeNotFound error.
    pub fn type_not_found(&mut self, range: Range, phase: Phase, path: &str) {
        self.emit(
            EK::TypeNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(path.to_string()),
        );
    }

    /// Wrapper function for DependencyCycle error.
    pub fn dep_cycle(&mut self, range: Range, phase: Phase, path: &str) {
        self.emit(
            EK::DependencyCycle,
            range,
            phase,
            Sv::Error,
            Pl::Str(path.to_string()),
        );
    }

    /// Wrapper function for OpCannotBeChained error.
    pub fn op_chained(&mut self, range: Range, phase: Phase, op: &str) {
        self.emit(
            EK::OpCannotBeChained,
            range,
            phase,
            Sv::Error,
            Pl::Str(op.to_string()),
        );
    }

    /// Wrapper function for UnmatchedParentheses error.
    pub fn unmatched_parentheses(&mut self, range: Range, phase: Phase) {
        self.emit(EK::UnmatchedParentheses, range, phase, Sv::Error, Pl::None);
    }

    /// Wrapper function for ArityMismatch error.
    pub fn arity_mismatch(&mut self, range: Range, phase: Phase, expected: usize, actual: usize) {
        self.emit(
            EK::ArityMismatch,
            range,
            phase,
            Sv::Error,
            Pl::NumPair(expected, actual),
        );
    }

    /// Wrapper function for InvalidExprSyntax error.
    pub fn invalid_expr_syntax(&mut self, range: Range, phase: Phase) {
        self.emit(EK::InvalidExprSyntax, range, phase, Sv::Error, Pl::None);
    }

    /// Wrapper function for ParamWithoutType error.
    pub fn param_without_type(&mut self, range: Range, phase: Phase, path: &str) {
        self.emit(
            EK::ParamWithoutType,
            range,
            phase,
            Sv::Error,
            Pl::Str(path.to_string()),
        );
    }

    /// Wrapper function for InvalidParamNumbersForInfix error.
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

    /// Wrapper function for InvalidParamNumbersForPrefix error.
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

    /// Wrapper function for OperatorHasDefaultValue error.
    pub fn op_def_val(&mut self, range: Range, phase: Phase, symbol: &str) {
        self.emit(
            EK::OpCannotHaveDefaultValue,
            range,
            phase,
            Sv::Error,
            Pl::Str(symbol.to_string()),
        );
    }

    /// Wrapper function for TypeMismatch error.
    pub fn type_mismatch(
        &mut self,
        range: Range,
        phase: Phase,
        annotation_type: &str,
        expression_type: &str,
    ) {
        self.emit(
            EK::TypeMismatch,
            range,
            phase,
            Sv::Error,
            Pl::StrPair(annotation_type.to_string(), expression_type.to_string()),
        );
    }
}
