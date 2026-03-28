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
    ast::Range,
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
