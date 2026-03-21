use crate::{
    Range,
    error::{EK, ErrorCollector, Phase, Pl, Sv},
};

impl ErrorCollector {
    pub(crate) fn no_type_annotation_or_def_val(&mut self, range: Range, phase: Phase) {
        self.emit(
            EK::NoTypeAnnotationOrDefVal,
            range,
            phase,
            Sv::Error,
            Pl::None,
        );
    }

    pub(crate) fn var_not_found(&mut self, range: Range, phase: Phase, var_name: &str) {
        self.emit(
            EK::VarNotFound,
            range,
            phase,
            Sv::Error,
            Pl::Str(var_name.to_string()),
        );
    }

    pub(crate) fn type_annotation_mismatch(
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
            Pl::StrPair(annotation_type, expr_type),
        );
    }

    pub(crate) fn duplicate_name(&mut self, range: Range, phase: Phase, name: &str) {
        self.emit(
            EK::DuplicateName,
            range,
            phase,
            Sv::Error,
            Pl::Str(name.to_string()),
        );
    }

    pub(crate) fn immutable_assignment(&mut self, range: Range, phase: Phase, var_name: &str) {
        self.emit(
            EK::ImmutableAssignment,
            range,
            phase,
            Sv::Error,
            Pl::Str(var_name.to_string()),
        );
    }

    pub(crate) fn static_var_access(&mut self, range: Range, phase: Phase) {
        self.emit(EK::StaticVarAccess, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn builtin_var_access(&mut self, range: Range, phase: Phase) {
        self.emit(EK::BuiltinVarAccess, range, phase, Sv::Error, Pl::None);
    }

    pub(crate) fn non_constant_for_array_count(&mut self, range: Range, phase: Phase) {
        self.emit(
            EK::NonConstantForArrayCount,
            range,
            phase,
            Sv::Error,
            Pl::None,
        );
    }

    pub(crate) fn non_constant_for_loop_count(&mut self, range: Range, phase: Phase) {
        self.emit(
            EK::NonConstantForLoopCount,
            range,
            phase,
            Sv::Error,
            Pl::None,
        );
    }
}
