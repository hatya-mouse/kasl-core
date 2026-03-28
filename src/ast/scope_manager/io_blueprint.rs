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

use crate::ast::{Expr, VariableID, type_registry::ResolvedType};
use std::collections::HashMap;

#[derive(Default)]
pub struct IOBlueprint {
    inputs: Vec<VariableID>,
    outputs: Vec<VariableID>,
    states: Vec<VariableID>,
    items: HashMap<VariableID, BlueprintItem>,
    decl_order: Vec<(VariableID, BlueprintItemKind)>,
}

pub struct BlueprintItem {
    pub name: String,
    pub actual_size: usize,
    pub align: u8,
    pub value_type: ResolvedType,
    pub def_val: Expr,
    pub id: VariableID,
}

pub(crate) enum BlueprintItemKind {
    Input,
    Output,
    State,
}

impl IOBlueprint {
    pub fn add_input(&mut self, item: BlueprintItem) {
        self.decl_order.push((item.id, BlueprintItemKind::Input));
        self.inputs.push(item.id);
        self.items.insert(item.id, item);
    }

    pub fn add_output(&mut self, item: BlueprintItem) {
        self.decl_order.push((item.id, BlueprintItemKind::Output));
        self.outputs.push(item.id);
        self.items.insert(item.id, item);
    }

    pub fn add_state(&mut self, item: BlueprintItem) {
        self.decl_order.push((item.id, BlueprintItemKind::State));
        self.states.push(item.id);
        self.items.insert(item.id, item);
    }

    pub fn get_inputs(&self) -> Vec<&BlueprintItem> {
        self.inputs
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect::<Vec<_>>()
    }

    pub fn get_outputs(&self) -> Vec<&BlueprintItem> {
        self.outputs
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect::<Vec<_>>()
    }

    pub fn get_states(&self) -> Vec<&BlueprintItem> {
        self.states
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect::<Vec<_>>()
    }

    pub(crate) fn get_item(&self, id: &VariableID) -> Option<&BlueprintItem> {
        self.items.get(id)
    }
}
