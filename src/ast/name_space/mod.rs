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

mod reserved_type_names;
mod symbol_id;
mod symbol_path;

pub use reserved_type_names::is_reserved_type_name;
pub use symbol_id::{FunctionID, OperatorID, ParserStmtID, StructID, VariableID};
pub use symbol_path::{SymbolPath, SymbolPathComponent};

#[derive(Debug, Default)]
pub struct NameSpace {
    next_variable_id: usize,
    next_struct_id: usize,
    next_function_id: usize,
    next_operator_id: usize,
}

impl NameSpace {
    pub fn generate_variable_id(&mut self) -> VariableID {
        let id = VariableID::new(self.next_variable_id);
        self.next_variable_id += 1;
        id
    }

    pub fn generate_struct_id(&mut self) -> StructID {
        let id = StructID::new(self.next_struct_id);
        self.next_struct_id += 1;
        id
    }

    pub fn generate_function_id(&mut self) -> FunctionID {
        let id = FunctionID::new(self.next_function_id);
        self.next_function_id += 1;
        id
    }

    pub fn generate_operator_id(&mut self) -> OperatorID {
        let id = OperatorID::new(self.next_operator_id);
        self.next_operator_id += 1;
        id
    }
}
