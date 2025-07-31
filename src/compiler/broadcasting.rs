//
// Copyright 2025 Shuntaro Kasatani
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

use crate::{TYPE_INT, Translator, compiler::codegen::get_ir_type};
use cranelift_codegen::{
    entity::EntityRef,
    ir::{
        self, DynamicStackSlotData, DynamicType, DynamicTypeData, DynamicTypes, GlobalValue,
        InstBuilder, MemFlags, StackSlotData, StackSlotKind, condcodes::IntCC, dynamic_type,
    },
    packed_option::ReservedValue,
};
use cranelift_jit::JITModule;
use cranelift_module::Module;

impl<'a> Translator<'a> {
    /// # Array type definition:
    /// ## Value part
    /// - `ptr: pointer` The pointer to the array data.
    ///
    /// ## Data part
    /// - `size: int` The size of the array (length).
    /// - `[data: pointer]` Content of the array which is the pointer to the each data.
    pub fn codegen_broadcast_op<F>(
        &mut self,
        mut op: F,
        vals: Vec<ir::Value>,
        val_types: Vec<knodiq_engine::Type>,
        module: &JITModule,
    ) -> (ir::Value, knodiq_engine::Type)
    where
        F: FnMut(
            Vec<ir::Value>,
            Vec<knodiq_engine::Type>,
            &mut Translator,
            &JITModule,
        ) -> ir::Value,
    {
        let pointer_type = module.target_config().pointer_type();

        let mut shapes = Vec::new();
        for i in 0..vals.len() {
            let shape = self.get_shape(vals[i], val_types[i].clone(), module);
            shapes.push(shape);
        }

        let target_shape_slot = self
            .builder
            .create_dynamic_stack_slot(DynamicStackSlotData::new(
                StackSlotKind::ExplicitDynamicSlot,
                ,
            ));
        let target_shape = self
            .builder
            .ins()
            .dynamic_stack_addr(pointer_type, target_shape_slot);
        // Store the size zero
        let zero = self.builder.ins().iconst(TYPE_INT, 0);
        self.builder
            .ins()
            .store(MemFlags::new(), zero, target_shape, 0);

        // for shape in shapes.iter().rev() {
        //     let target_len_val = self.load_arr_size(target_shape);
        //     let shape_len_val = self.builder.ins().iconst(TYPE_INT, shape.len() as i64);
        //     let cond =
        //         self.builder
        //             .ins()
        //             .icmp(IntCC::SignedGreaterThan, shape_len_val, target_len_val);

        //     fn then_fn(
        //         slf: &mut Translator,
        //         shape_len_val: ir::Value,
        //         target_shape: ir::Value,
        //         shape: &Vec<ir::Value>,
        //     ) -> Vec<(cranelift_codegen::ir::Value, cranelift_codegen::ir::Type)> {
        //         slf.builder
        //             .ins()
        //             .store(MemFlags::new(), shape_len_val, target_shape, 0);

        //         // Store the shape dimensions
        //         for dim in shape {
        //             slf.builder
        //                 .ins()
        //                 .store(MemFlags::new(), *dim, target_shape, 0);
        //         }

        //         vec![]
        //     }

        //     fn else_fn(
        //         slf: &mut Translator,
        //         shape_len_val: ir::Value,
        //         target_len_val: ir::Value,
        //         target_shape: ir::Value,
        //         shape: &Vec<ir::Value>,
        //     ) -> Vec<(cranelift_codegen::ir::Value, cranelift_codegen::ir::Type)> {
        //         let cond = slf
        //             .builder
        //             .ins()
        //             .icmp(IntCC::Equal, shape_len_val, target_len_val);

        //         fn inner_then_fn(
        //             slf: &mut Translator,
        //             target_shape: ir::Value,
        //             shape: &Vec<ir::Value>,
        //         ) -> Vec<(cranelift_codegen::ir::Value, cranelift_codegen::ir::Type)>
        //         {
        //             for (i, dim) in shape.iter().enumerate() {
        //                 let i_val = slf.builder.ins().iconst(TYPE_INT, i as i64);
        //                 let target_dim = slf.load_arr_val_at(TYPE_INT, target_shape, i_val);
        //                 let cond =
        //                     slf.builder
        //                         .ins()
        //                         .icmp(IntCC::SignedGreaterThan, *dim, target_dim);

        //                 fn inner_inner_then_fn(
        //                     slf: &mut Translator,
        //                     dim: ir::Value,
        //                     target_shape: ir::Value,
        //                     i: usize,
        //                 ) -> Vec<(cranelift_codegen::ir::Value, cranelift_codegen::ir::Type)>
        //                 {
        //                     // Calculate the offset to store the dimension at
        //                     // Considering the size which is contained in the first part
        //                     let data_offset = TYPE_INT.bytes() as i32;
        //                     let target_offset = data_offset + i as i32 * TYPE_INT.bytes() as i32;
        //                     slf.builder.ins().store(
        //                         MemFlags::new(),
        //                         dim,
        //                         target_shape,
        //                         target_offset,
        //                     );
        //                     vec![]
        //                 }

        //                 slf.codegen_if(
        //                     cond,
        //                     |slf| inner_inner_then_fn(slf, *dim, target_shape, i),
        //                     |_| vec![],
        //                 );
        //             }
        //             vec![]
        //         }

        //         slf.codegen_if(
        //             cond,
        //             |slf| inner_then_fn(slf, target_shape, shape),
        //             |_| vec![],
        //         );

        //         vec![]
        //     }

        //     self.codegen_if(
        //         cond,
        //         |slf| then_fn(slf, shape_len_val, target_shape, shape),
        //         |slf| else_fn(slf, shape_len_val, target_len_val, target_shape, shape),
        //     );
        // }

        // let reshaped_vals = vals
        //     .into_iter()
        //     .enumerate()
        //     .map(|(i, val)| {
        //         let shape_addr = self.builder.ins().get_frame_pointer(pointer_type);
        //         // Store the shape size
        //         let shape_size = self.builder.ins().iconst(TYPE_INT, shapes[i].len() as i64);
        //         self.builder.ins().store(
        //             MemFlags::new(),
        //             shape_size,
        //             shape_addr,
        //             TYPE_INT.bytes() as i32,
        //         );
        //         // Store the shape dimensions
        //         for (j, dim) in shapes[i].iter().enumerate() {
        //             let dim_offset = TYPE_INT.bytes() as i32 + j as i32 * TYPE_INT.bytes() as i32;
        //             self.builder
        //                 .ins()
        //                 .store(MemFlags::new(), *dim, shape_addr, dim_offset);
        //         }

        //         self.reshape(val, shape_addr, target_shape, module)
        //     })
        //     .collect::<Vec<ir::Value>>();

        // (
        //     target_shape,
        //     knodiq_engine::Type::Array(Box::new(knodiq_engine::Type::Int)),
        // )

        (ir::Value::from_u32(0), knodiq_engine::Type::Float)
    }

    pub fn get_shape(
        &mut self,
        val: ir::Value,
        val_type: knodiq_engine::Type,
        module: &JITModule,
    ) -> Vec<ir::Value> {
        let depth = val_type.get_depth();
        let mut shape = Vec::new();
        let mut current_arr = val;
        let mut current_type = val_type;
        let zero = self.builder.ins().iconst(TYPE_INT, 0);

        for _ in 0..depth {
            shape.push(self.load_arr_size(current_arr));
            if let knodiq_engine::Type::Array(inner_type) = current_type {
                current_arr =
                    self.load_arr_val_at(get_ir_type(&inner_type, module), current_arr, zero);
                current_type = *inner_type;
            }
        }

        shape
    }

    pub fn reshape(
        &mut self,
        mut arr: ir::Value,
        original_shape: ir::Value,
        target_shape: ir::Value,
        module: &JITModule,
    ) -> ir::Value {
        let pointer_type = module.target_config().pointer_type();

        let original_depth = self.load_arr_size(original_shape);
        let target_depth = self.load_arr_size(target_shape);
        let depth_diff = self.builder.ins().isub(original_depth, target_depth);

        self.codegen_loop(depth_diff, |slf, _| {
            // Wrap the array in a new array
            let old_arr = arr;
            arr = slf.builder.ins().get_frame_pointer(pointer_type);

            // Store the length
            let arr_size = slf.builder.ins().iconst(TYPE_INT, 1);
            slf.builder.ins().store(MemFlags::new(), arr_size, arr, 0);
            // Then the pointer to the old array
            slf.builder.ins().store(MemFlags::new(), old_arr, arr, 0);
        });

        arr
    }
}
