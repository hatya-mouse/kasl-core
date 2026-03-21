use crate::{
    Range,
    error::{EK, ErrorCollector, Phase, Pl, Sv},
};

impl ErrorCollector {
    pub(crate) fn import_not_found(&mut self, range: Range, phase: Phase, path: String) {
        self.emit(EK::ImportNotFound, range, phase, Sv::Error, Pl::Str(path));
    }

    pub(crate) fn cyclic_dependency(&mut self, range: Range, phase: Phase, path: String) {
        self.emit(EK::CyclicDependency, range, phase, Sv::Error, Pl::Str(path));
    }
}
