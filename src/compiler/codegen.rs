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

use crate::{Expression, Operator, Statement, SymbolInfo, TYPE_FLOAT, TYPE_INT};
use cranelift_codegen::{
    entity::EntityRef,
    ir::{self, Block, BlockArg, InstBuilder, types},
};
use cranelift_frontend::{FunctionBuilder, Variable};
use cranelift_jit::JITModule;
use cranelift_module::Module;
use std::collections::HashMap;

pub struct Translator<'a> {
    pub builder: FunctionBuilder<'a>,
    pub variables: HashMap<String, (Variable, ir::Type)>,
    pub return_vars: Vec<String>,
    pub functions: HashMap<String, ir::FuncRef>,
    entry_block: Block,
    input_ptr: ir::Value,
    output_ptr: ir::Value,
    input_count: ir::Value,
    output_count: ir::Value,
}

impl<'a> Translator<'a> {
    /// Creates a new `Translator` instance with the given function builder, functions, and entry block.
    pub fn new(
        mut builder: FunctionBuilder<'a>,
        functions: HashMap<String, ir::FuncRef>,
        entry_block: Block,
    ) -> Self {
        let input_ptr = builder.ins().iconst(TYPE_INT, 0);
        let output_ptr = builder.ins().iconst(TYPE_INT, 0);
        let input_count = builder.ins().iconst(TYPE_INT, 0);
        let output_count = builder.ins().iconst(TYPE_INT, 0);

        Translator {
            builder,
            variables: HashMap::new(),
            return_vars: Vec::new(),
            functions,
            entry_block,
            input_ptr,
            output_ptr,
            input_count,
            output_count,
        }
    }

    /// Sets up the array interface for the function.
    pub fn setup_array_interface(
        &mut self,
        inputs: &Vec<SymbolInfo>,
        outputs: &Vec<SymbolInfo>,
        module: &JITModule,
    ) {
        let params = self.builder.block_params(self.entry_block);
        self.input_ptr = params[0];
        self.input_count = params[1];
        self.output_ptr = params[2];
        self.output_count = params[3];

        let mut input_offset = 0;
        for input_info in inputs {
            let var = Variable::new(self.variables.len());
            let val_type = get_type(&input_info.value_type, module);

            let offset = self.builder.ins().iconst(TYPE_INT, input_offset);
            let addr = self.builder.ins().iadd(self.input_ptr, offset);
            let val = self
                .builder
                .ins()
                .load(val_type, ir::MemFlags::new(), addr, 0);

            self.variables
                .insert(input_info.name.clone(), (var, val_type));
            self.builder.declare_var(var, val_type);
            self.builder.def_var(var, val);

            // Calculate the offset for the next element
            input_offset += val_type.bytes() as i64;
        }

        for output_info in outputs {
            let var = Variable::new(self.variables.len());
            let val_type = get_type(&output_info.value_type, module);

            self.variables
                .insert(output_info.name.clone(), (var, val_type));
            self.return_vars.push(output_info.name.clone());
            self.builder.declare_var(var, val_type);

            let default_val = match val_type {
                TYPE_INT => self.builder.ins().iconst(TYPE_INT, 0),
                types::F32 => self.builder.ins().f32const(0.0),
                _ => self.builder.ins().iconst(val_type, 0),
            };

            self.builder.def_var(var, default_val);
        }
    }

    /// Finalizes the array interface by storing the output values into the output pointer.
    pub fn finalize_array_interface(&mut self, output_names: &[String]) {
        let mut output_offset = 0;
        for output_name in output_names.iter() {
            if let Some((var, val_type)) = self.variables.get(output_name) {
                let val = self.builder.use_var(*var);
                let offset = self.builder.ins().iconst(TYPE_INT, output_offset);
                let addr = self.builder.ins().iadd(self.output_ptr, offset);
                self.builder.ins().store(ir::MemFlags::new(), val, addr, 0);

                // Calculate the offset for the next element
                output_offset += val_type.bytes() as i64;
            }
        }

        self.builder.ins().return_(&[]);
    }

    /// Generates code for a statement.
    pub fn codegen_stmt(&mut self, statement: &Statement, module: &JITModule) {
        match statement {
            Statement::VariableDeclaration(var_decl) => {
                let var = Variable::new(self.variables.len());
                let (val, val_type) = self.codegen_expr(&var_decl.initial_value, module);

                self.variables
                    .insert(var_decl.name.clone(), (var, val_type));

                self.builder.declare_var(var, val_type);
                self.builder.def_var(var, val);
            }
            Statement::Assignment(assignment_stmt) => {
                let (val, _) = self.codegen_expr(&assignment_stmt.value, module);
                let var = self
                    .variables
                    .get(&assignment_stmt.target_name)
                    .expect("Variable not found")
                    .0;
                self.builder.def_var(var, val);
            }
            Statement::ForLoop(_for_loop_stmt) => {
                // self.codegen_loop(for_loop_stmt.count, for_loop_stmt.body)
            }
            _ => return,
        }
    }

    /// Converts a `knodiq_engine::Value` to an IR value.
    pub fn value_as_ir(&mut self, value: &knodiq_engine::Value, module: &JITModule) -> ir::Value {
        match value {
            knodiq_engine::Value::Int(i) => self.builder.ins().iconst(TYPE_INT, *i as i64),
            knodiq_engine::Value::Float(f) => self.builder.ins().f32const(*f),
            knodiq_engine::Value::Array(arr) => {
                let vals = arr.iter().map(|v| self.value_as_ir(v, module)).collect();
                self.vec_as_array(vals, module)
            }
        }
    }

    /// Generates code for an expression and returns the resulting IR value and its type.
    pub fn codegen_expr(&mut self, expr: &Expression, module: &JITModule) -> (ir::Value, ir::Type) {
        println!("Codegen expression: {:?}", expr);
        match expr {
            Expression::IntLiteral(lit) => {
                (self.builder.ins().iconst(TYPE_INT, *lit as i64), TYPE_INT)
            }
            Expression::FloatLiteral(lit) => (self.builder.ins().f32const(*lit), TYPE_FLOAT),
            Expression::Identifier(id) => {
                let (var, var_type) = self.variables.get(id).expect("Variable not found");
                (self.builder.use_var(*var), *var_type)
            }
            Expression::BinaryOp {
                op,
                left,
                right,
                left_type,
                right_type,
            } => {
                let (left_val, left_ir_type) = self.codegen_expr(left, module);
                let (right_val, right_ir_type) = self.codegen_expr(right, module);
                let op_type = eval_type(left_ir_type, right_ir_type);
                (
                    self.codegen_op(
                        op,
                        left_val,
                        right_val,
                        left_type.clone(),
                        right_type.clone(),
                        module,
                    ),
                    op_type,
                )
            }
            Expression::FunctionCall { name, arguments } => {
                let func = *self.functions.get(name).expect("Function not found");
                let mut args = Vec::with_capacity(arguments.len());
                for arg in arguments {
                    args.push(self.codegen_expr(arg, module).0);
                }

                let call = self.builder.ins().call(func, &args);
                let results = self.builder.inst_results(call);
                let result_val = results[0];
                (result_val, TYPE_FLOAT)
            }
        }
    }

    /// Generates code for a binary operation based on the operator and types of the left and right values.
    pub fn codegen_op(
        &mut self,
        op: &Operator,
        left: ir::Value,
        right: ir::Value,
        left_type: knodiq_engine::Type,
        right_type: knodiq_engine::Type,
        module: &JITModule,
    ) -> ir::Value {
        match (left_type, right_type) {
            (knodiq_engine::Type::Int, knodiq_engine::Type::Int) => {
                self.codegen_int_op(op, left, right)
            }
            (knodiq_engine::Type::Float, knodiq_engine::Type::Float) => {
                self.codegen_float_op(op, left, right)
            }
            (left_type, right_type) => {
                self.codegen_broadcast_op(op, left, right, left_type, right_type, module)
            }
        }
    }

    /// Performs an integer operation based on the operator.
    pub fn codegen_int_op(
        &mut self,
        op: &Operator,
        left: ir::Value,
        right: ir::Value,
    ) -> ir::Value {
        match op {
            Operator::Add => self.builder.ins().iadd(left, right),
            Operator::Subtract => self.builder.ins().isub(left, right),
            Operator::Multiply => self.builder.ins().imul(left, right),
            Operator::Divide => self.builder.ins().udiv(left, right),
            Operator::Modulo => self.builder.ins().urem(left, right),
        }
    }

    /// Performs a float operation based on the operator.
    pub fn codegen_float_op(
        &mut self,
        op: &Operator,
        left: ir::Value,
        right: ir::Value,
    ) -> ir::Value {
        match op {
            Operator::Add => self.builder.ins().fadd(left, right),
            Operator::Subtract => self.builder.ins().fsub(left, right),
            Operator::Multiply => self.builder.ins().fmul(left, right),
            Operator::Divide => self.builder.ins().fdiv(left, right),
            _ => left,
        }
    }

    /// Performs a broadcast operation for array types.
    /// This function can handle cases where the left and right values are of different depths.
    pub fn codegen_broadcast_op(
        &mut self,
        op: &Operator,
        mut left_val: ir::Value,
        mut right_val: ir::Value,
        mut left_type: knodiq_engine::Type,
        mut right_type: knodiq_engine::Type,
        module: &JITModule,
    ) -> ir::Value {
        let left_depth = left_type.get_depth();
        let right_depth = right_type.get_depth();

        while left_depth > right_depth {
            right_val = self.vec_as_array(vec![left_val], module);
            right_type = knodiq_engine::Type::Array(Box::new(right_type));
        }

        while left_depth < right_depth {
            left_val = self.vec_as_array(vec![right_val], module);
            left_type = knodiq_engine::Type::Array(Box::new(left_type));
        }

        self.recurse_op(op, left_val, right_val, left_type, right_type, module)
    }

    /// Recursively applies the operation to each element of the arrays.
    pub fn recurse_op(
        &mut self,
        op: &Operator,
        left_val: ir::Value,
        right_val: ir::Value,
        left_type: knodiq_engine::Type,
        right_type: knodiq_engine::Type,
        module: &JITModule,
    ) -> ir::Value {
        match (left_type, right_type) {
            (
                knodiq_engine::Type::Array(left_inside_type),
                knodiq_engine::Type::Array(right_inside_type),
            ) => {
                let left_inside_ir_type = get_type(&left_inside_type, module);
                let right_inside_ir_type = get_type(&right_inside_type, module);

                let left_vals = self.load_arr_from(left_inside_ir_type, left_val, module);
                let right_vals = self.load_arr_from(right_inside_ir_type, right_val, module);

                let mut return_val = ir::Value::new(0);
                for left_vals in left_vals.iter() {
                    for right_vals in right_vals.iter() {
                        return_val = self.codegen_op(
                            op,
                            *left_vals,
                            *right_vals,
                            left_inside_type.as_ref().clone(),
                            right_inside_type.as_ref().clone(),
                            module,
                        );
                    }
                }

                return_val
            }
            _ => self.codegen_float_op(op, left_val, right_val),
        }
    }

    /// Converts a vector of IR values into an array pointer.
    /// This function assumes that the values are of the same type.
    ///
    /// # Array type definition:
    /// ## Value part
    /// - `ptr: pointer` The pointer to the array data.
    ///
    /// ## Data part
    /// - `size: int` The size of the array (length).
    /// - `[data: pointer]` Content of the array which is the pointer to the each data.
    pub fn vec_as_array(&mut self, elems: Vec<ir::Value>, module: &JITModule) -> ir::Value {
        let ptr_type = module.target_config().pointer_type();
        let arr_ptr = self.builder.ins().get_stack_pointer(ptr_type);
        self.builder.ins().store(
            ir::MemFlags::new(),
            ir::Value::from_u32(elems.len() as u32),
            arr_ptr,
            0,
        );
        for (i, elem) in elems.iter().enumerate() {
            self.builder
                .ins()
                .store(ir::MemFlags::new(), *elem, arr_ptr, 1 + i as i32);
        }
        arr_ptr
    }

    /// Take outs IR values from an array pointer.
    ///
    /// Array stores pointers for each data so this function loads each data from the pointer.
    pub fn load_arr_from(
        &mut self,
        val_type: ir::Type,
        addr: ir::Value,
        module: &JITModule,
    ) -> Vec<ir::Value> {
        let current_block = self.builder.current_block().unwrap();
        let ptr_type = module.target_config().pointer_type();

        let data_bytes = self.builder.ins().iconst(TYPE_INT, TYPE_INT.bytes() as i64);

        // Get the size of the array
        let size = self
            .builder
            .ins()
            .load(ptr_type, ir::MemFlags::new(), addr, 0);

        // Calculate the starting address of the data by adding the size of the size
        let data_offset = self.builder.ins().iconst(TYPE_INT, TYPE_INT.bytes() as i64);

        let mut vals = Vec::new();

        // LOOP
        let loop_block = self.builder.create_block();
        let next_block = self.builder.create_block();
        self.builder.append_block_param(loop_block, TYPE_INT);

        self.builder
            .ins()
            .jump(loop_block, &[BlockArg::Value(data_offset)]);
        self.builder.switch_to_block(loop_block);

        self.builder.insert_block_after(loop_block, current_block);
        self.builder.insert_block_after(next_block, current_block);

        // --- LOOP BODY ---
        let i = self.builder.block_params(loop_block)[0];

        // Calculate the address of the current element
        let offset_bytes = self.builder.ins().imul(i, data_bytes);
        let target_addr = self.builder.ins().iadd(addr, offset_bytes);

        // Load the pointer to the current element
        let current_ptr = self
            .builder
            .ins()
            .load(ptr_type, ir::MemFlags::new(), target_addr, 0);

        // Get the value from the pointer
        let current_val = self
            .builder
            .ins()
            .load(val_type, ir::MemFlags::new(), current_ptr, 0);
        vals.push(current_val);

        // --- LOOP CONDITION ---
        let cmp = self
            .builder
            .ins()
            .icmp(ir::condcodes::IntCC::SignedLessThan, i, size);
        let next_i = self.builder.ins().iadd(i, ir::Value::from_u32(1));
        self.builder
            .ins()
            .brif(cmp, loop_block, &[BlockArg::Value(next_i)], next_block, []);

        self.builder.switch_to_block(next_block);

        vec![ir::Value::new(0)]
    }

    /// Loads an array value at a specific index from the given address.
    pub fn load_arr_val_at(
        &mut self,
        val_type: ir::Type,
        addr: ir::Value,
        index: ir::Value,
    ) -> ir::Value {
        let type_bytes = self.builder.ins().iconst(TYPE_INT, val_type.bytes() as i64);

        let offset_bytes = self.builder.ins().imul(index, type_bytes);
        let target_addr = self.builder.ins().iadd(addr, offset_bytes);
        let current_val = self
            .builder
            .ins()
            .load(val_type, ir::MemFlags::new(), target_addr, 0);

        current_val
    }

    /// Generates code for a loop that iterates `count` times, executing the body for each iteration.
    pub fn codegen_loop(&mut self, count: ir::Value, mut body: Box<dyn FnMut(ir::Value)>) -> Block {
        let current_block = self.builder.current_block().unwrap();

        // Define blocks
        let loop_block = self.builder.create_block();
        let next_block = self.builder.create_block();
        self.builder.append_block_param(loop_block, TYPE_INT);
        self.builder.insert_block_after(loop_block, current_block);
        self.builder.insert_block_after(next_block, current_block);

        let start_i = ir::Value::from_u32(0);
        self.builder
            .ins()
            .jump(loop_block, [&BlockArg::Value(start_i)]);
        self.builder.switch_to_block(loop_block);

        // LOOP BODY
        let i = self.builder.block_params(loop_block)[0];
        body(i);

        // Loop condition
        let cmp = self
            .builder
            .ins()
            .icmp(ir::condcodes::IntCC::SignedLessThan, i, count);
        let next_i = self.builder.ins().iadd(i, ir::Value::from_u32(1));
        self.builder
            .ins()
            .brif(cmp, loop_block, &[BlockArg::Value(next_i)], next_block, []);

        next_block
    }
}

/// Converts a `knodiq_engine::Type` to an IR type.
pub fn get_type(value_type: &knodiq_engine::Type, module: &JITModule) -> types::Type {
    match value_type {
        knodiq_engine::Type::Int => TYPE_INT,
        knodiq_engine::Type::Float => TYPE_FLOAT,
        knodiq_engine::Type::Array(_) => module.target_config().pointer_type(),
        knodiq_engine::Type::None => types::INVALID,
    }
}

/// Evaluates the resulting type of a binary operation based on the left and right types.
pub fn eval_type(left: ir::Type, right: ir::Type) -> ir::Type {
    match left {
        TYPE_INT => match right {
            TYPE_INT => TYPE_INT,
            types::F32 => types::F32,
            types::INVALID => right,
            _ => right,
        },
        types::F32 => match right {
            TYPE_INT => types::F32,
            types::F32 => types::F32,
            types::INVALID => left,
            _ => right,
        },
        types::INVALID => right,
        _ => left,
    }
}
