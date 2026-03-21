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

mod item_store;

use crate::{
    backend::func_translator::{FuncTranslator, TranslatorParams},
    scope_manager::IOBlueprint,
};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn store_blueprint(
        &mut self,
        params: &TranslatorParams,
        blueprint: &IOBlueprint,
        iteration_index: Option<ir::Value>,
    ) {
        // Get the type of a pointer
        let pointer_type = self.type_converter.pointer_type();

        // Store outputs and states
        self.store_outputs(
            pointer_type,
            params.output_ptr_ptr,
            blueprint,
            iteration_index,
        );
        self.store_states(pointer_type, params.state_ptr_ptr, blueprint);
    }

    fn store_outputs(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        blueprint: &IOBlueprint,
        iteration_index: Option<ir::Value>,
    ) {
        let mut output_offset: usize = 0;
        for output_item in blueprint.get_outputs() {
            self.store_blueprint_item(
                pointer_type,
                ptr_ptr,
                output_item,
                output_offset as i32,
                iteration_index,
            );
            // Increment the output offset by the size of a pointer
            // because each output is stored as a pointer to the actual value
            output_offset += pointer_type.bytes() as usize;
        }
    }

    fn store_states(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        blueprint: &IOBlueprint,
    ) {
        let mut state_offset: usize = 0;
        for state_item in blueprint.get_states() {
            // States shares the memory over the loop so we do not need to pass a sample index
            self.store_blueprint_item(pointer_type, ptr_ptr, state_item, state_offset as i32, None);
            // Increment the state offset by the size of a pointer
            state_offset += pointer_type.bytes() as usize;
        }
    }
}
