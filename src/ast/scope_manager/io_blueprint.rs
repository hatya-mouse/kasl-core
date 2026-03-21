use std::collections::HashMap;

use crate::{Expr, VariableID, type_registry::ResolvedType};

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

    pub(crate) fn get_order(&self) -> &[(VariableID, BlueprintItemKind)] {
        &self.decl_order
    }

    pub(crate) fn get_item(&self, id: &VariableID) -> Option<&BlueprintItem> {
        self.items.get(id)
    }
}
