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

mod import_path;
mod reserved_type_names;
mod symbol_id;
mod symbol_path;

pub use import_path::ImportPath;
pub use reserved_type_names::is_reserved_type_name;
pub use symbol_id::{FunctionID, OperatorID, ParserStmtID, StructID, VariableID};
pub use symbol_path::{SymbolPath, SymbolPathComponent};

use crate::ProgramContext;

#[derive(Debug, Default)]
pub struct NameSpace {
    pub name: String,
    pub prog_ctx: ProgramContext,
}
