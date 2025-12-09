//
// Copyright 2025 Shuntaro Kasatani
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

use crate::{ConstructorError, ConstructorErrorType, Function, Program, Range, SymbolPath};

impl Program {
    pub fn register_func_by_path(
        &mut self,
        func: Function,
        to_path: SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::TypeNotFound(to_path),
                    position: Range::zero(),
                });
            }
        };

        target_scope.register_func(func);

        Ok(())
    }

    // Implement other member registration methods here
}
