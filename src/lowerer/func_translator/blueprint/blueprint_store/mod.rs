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

mod item_store;

use crate::{
    ast::scope_manager::IOBlueprint,
    lowerer::{TranslatorParams, func_translator::FuncTranslator},
};
use kasl_ir::ir::{Offset, Value};

impl FuncTranslator<'_> {
    pub fn store_blueprint(
        &mut self,
        params: &TranslatorParams,
        blueprint: &IOBlueprint,
        iteration: Option<Value>,
    ) {
        // Store outputs and states
        self.store_outputs(params.output_ptr_ptr, blueprint, iteration);
        self.store_states(params.state_ptr_ptr, blueprint);
    }

    fn store_outputs(&mut self, ptr_ptr: Value, blueprint: &IOBlueprint, iteration: Option<Value>) {
        for (output_count, output_item) in (0u32..).zip(blueprint.get_outputs()) {
            // Loop through the output items and store the value to the output pointer
            let offset = Offset::PointerScaled(output_count);
            self.store_blueprint_item(ptr_ptr, output_item, offset, iteration);
        }
    }

    fn store_states(&mut self, ptr_ptr: Value, blueprint: &IOBlueprint) {
        for (state_count, state_item) in (0u32..).zip(blueprint.get_states()) {
            // Loop through the state variables and store the value to the state pointer
            let offset = Offset::PointerScaled(state_count);
            self.store_blueprint_item(ptr_ptr, state_item, offset, None);
        }
    }
}
