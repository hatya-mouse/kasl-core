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
    backend::func_translator::FuncTranslator,
    scope_manager::{BlueprintItem, IOBlueprint},
};
use cranelift::prelude::{InstBuilder, MemFlags, types};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn store_blueprint(
        &mut self,
        output_ptr_ptr: ir::Value,
        state_ptr_ptr: ir::Value,
        blueprint: &IOBlueprint,
    ) {
        // Get the type of a pointer
        let pointer_type = self.type_converter.pointer_type();

        // OUTPUTS
        let mut output_offset: usize = 0;
        for output_item in blueprint.get_outputs() {
            self.store_blueprint_item(
                pointer_type,
                output_ptr_ptr,
                output_item,
                output_offset as i32,
            );
            // Increment the output offset by the size of a pointer
            // because each output is stored as a pointer to the actual value
            output_offset += pointer_type.bytes() as usize;
        }

        // STATES
        let mut state_offset: usize = 0;
        for state_item in blueprint.get_states() {
            self.store_blueprint_item(pointer_type, state_ptr_ptr, state_item, state_offset as i32);
            // Increment the state offset by the size of a pointer
            state_offset += pointer_type.bytes() as usize;
        }
    }

    fn store_blueprint_item(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        item: &BlueprintItem,
        offset: i32,
    ) {
        // get the pointer to the value by the pointer to the pointers
        let output_ptr = self
            .builder
            .ins()
            .load(pointer_type, MemFlags::new(), ptr_ptr, offset);

        // Get the value to store
        let var = self.variables.get(&item.id).unwrap();
        let val = self.builder.use_var(*var);

        // Store the value
        let val = if self.builder.func.dfg.value_type(val) == types::I8 {
            self.builder.ins().uextend(types::I32, val)
        } else {
            val
        };
        self.builder
            .ins()
            .store(MemFlags::new(), val, output_ptr, 0);
    }
}
