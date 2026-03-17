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

use crate::{Expr, VariableID, type_registry::ResolvedType};

#[derive(Default)]
pub struct IOBlueprint {
    inputs: Vec<BlueprintItem>,
    outputs: Vec<BlueprintItem>,
    states: Vec<BlueprintItem>,
}

pub struct BlueprintItem {
    pub name: String,
    pub actual_size: usize,
    pub align: u8,
    pub value_type: ResolvedType,
    pub def_val: Expr,
    pub id: VariableID,
}

impl IOBlueprint {
    pub fn add_input(&mut self, item: BlueprintItem) {
        self.inputs.push(item);
    }

    pub fn add_output(&mut self, item: BlueprintItem) {
        self.outputs.push(item);
    }

    pub fn add_state(&mut self, item: BlueprintItem) {
        self.states.push(item);
    }

    pub fn get_inputs(&self) -> &[BlueprintItem] {
        &self.inputs
    }

    pub fn get_outputs(&self) -> &[BlueprintItem] {
        &self.outputs
    }

    pub fn get_states(&self) -> &[BlueprintItem] {
        &self.states
    }
}
