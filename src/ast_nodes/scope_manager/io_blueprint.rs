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

use crate::ast_nodes::{Expr, VariableID, type_registry::ResolvedType};
use std::collections::HashMap;

/// Holds the desired memory layout and type information for the inputs, outputs, and states of a KASL program after `build` phase.
/// The host is expected to allocate memory for the inputs, outputs, and states based on the blueprint, and then pass the pointers when running the program.
#[derive(Default)]
pub struct IOBlueprint {
    inputs: Vec<VariableID>,
    outputs: Vec<VariableID>,
    states: Vec<VariableID>,
    items: HashMap<VariableID, BlueprintItem>,
    decl_order: Vec<(VariableID, BlueprintItemKind)>,
}

/// Represents an input, output, or state variable in the IOBlueprint, along with its type and memory layout information.
pub struct BlueprintItem {
    /// The name of the variable.
    pub name: String,
    /// The actual memory size of the variable, in bytes.
    pub actual_size: u32,
    /// The alignment of the variable, in bytes.
    pub align: u32,
    /// The type of the variable.
    pub value_type: ResolvedType,
    /// The default value of the variable, which will be used for initialization.
    /// The host does not need to provide any initial value for the variable.
    pub def_val: Expr,
    /// The unique identifier of the variable.
    pub id: VariableID,
}

pub(crate) enum BlueprintItemKind {
    Input,
    Output,
    State,
}

impl IOBlueprint {
    /// Adds an input variable to the blueprint.
    pub(crate) fn add_input(&mut self, item: BlueprintItem) {
        self.decl_order.push((item.id, BlueprintItemKind::Input));
        self.inputs.push(item.id);
        self.items.insert(item.id, item);
    }

    /// Adds an output variable to the blueprint.
    pub(crate) fn add_output(&mut self, item: BlueprintItem) {
        self.decl_order.push((item.id, BlueprintItemKind::Output));
        self.outputs.push(item.id);
        self.items.insert(item.id, item);
    }

    /// Adds a state variable to the blueprint.
    pub(crate) fn add_state(&mut self, item: BlueprintItem) {
        self.decl_order.push((item.id, BlueprintItemKind::State));
        self.states.push(item.id);
        self.items.insert(item.id, item);
    }

    /// Returns the input variables in the order they were declared.
    pub fn get_inputs(&self) -> Vec<&BlueprintItem> {
        self.inputs
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect::<Vec<_>>()
    }

    /// Returns the output variables in the order they were declared.
    pub fn get_outputs(&self) -> Vec<&BlueprintItem> {
        self.outputs
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect::<Vec<_>>()
    }

    /// Returns the state variables in the order they were declared.
    pub fn get_states(&self) -> Vec<&BlueprintItem> {
        self.states
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect::<Vec<_>>()
    }

    /// Returns the blueprint item with the given ID if it exists, or returns `None`.
    pub(crate) fn get_item(&self, id: &VariableID) -> Option<&BlueprintItem> {
        self.items.get(id)
    }
}
