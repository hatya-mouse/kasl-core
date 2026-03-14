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
    FuncParam, Range, VariableID,
    type_registry::{PrimitiveType, ResolvedType},
};

pub(in crate::builtin) fn func_param(
    label: Option<&str>,
    name: &str,
    value_type: PrimitiveType,
) -> FuncParam {
    FuncParam {
        label: label.map(str::to_string),
        name: name.to_string(),
        var_id: VariableID::default(),
        value_type: ResolvedType::Primitive(value_type),
        def_val: None,
        range: Range::zero(),
    }
}
