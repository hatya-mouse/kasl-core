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
use knodiq_engine::graph::value::type_of;
use std::collections::HashMap;

pub struct Translator<'a> {
    pub builder: FunctionBuilder<'a>,
    pub variables: HashMap<String, (Variable, knodiq_engine::Type)>,
    pub return_vars: Vec<String>,
    pub functions: HashMap<String, ir::FuncRef>,
    entry_block: Block,
    input_ptr: ir::Value,
    output_ptr: ir::Value,
    input_count: ir::Value,
    output_count: ir::Value,
    var_counter: usize,
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
            var_counter: 0,
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
            let var = self.new_var();
            let val_type = get_type(&input_info.value_type, module);

            let offset = self.builder.ins().iconst(TYPE_INT, input_offset);
            let addr = self.builder.ins().iadd(self.input_ptr, offset);
            let val = self
                .builder
                .ins()
                .load(val_type, ir::MemFlags::new(), addr, 0);

            self.variables.insert(
                input_info.name.clone(),
                (var, input_info.value_type.clone()),
            );
            self.builder.declare_var(var, val_type);
            self.builder.def_var(var, val);

            // Calculate the offset for the next element
            input_offset += val_type.bytes() as i64;
        }

        for output_info in outputs {
            let var = self.new_var();
            let val_type = get_type(&output_info.value_type, module);

            self.variables.insert(
                output_info.name.clone(),
                (var, output_info.value_type.clone()),
            );
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
    pub fn finalize_array_interface(&mut self, output_names: &[String], module: &JITModule) {
        let mut output_offset = 0;
        for output_name in output_names.iter() {
            if let Some((var, val_type)) = self.variables.get(output_name) {
                let val = self.builder.use_var(*var);
                let offset = self.builder.ins().iconst(TYPE_INT, output_offset);
                let addr = self.builder.ins().iadd(self.output_ptr, offset);
                self.builder.ins().store(ir::MemFlags::new(), val, addr, 0);

                // Calculate the offset for the next element
                output_offset += get_type(val_type, module).bytes() as i64;
            }
        }

        self.builder.ins().return_(&[]);
    }

    /// Generates code for a statement.
    pub fn codegen_stmt(&mut self, statement: &Statement, module: &JITModule) {
        println!("Codegen statement: {:?}", statement);
        match statement {
            Statement::VariableDeclaration(var_decl) => {
                let var = self.new_var();
                let (val, val_type) = self.codegen_expr(&var_decl.initial_value, module);

                self.variables
                    .insert(var_decl.name.clone(), (var, var_decl.value_type.clone()));

                self.builder.declare_var(var, get_type(&val_type, module));
                self.builder.def_var(var, val);
            }
            Statement::Assignment(assignment_stmt) => {
                let (val, val_type) = self.codegen_expr(&assignment_stmt.value, module);
                let val_ir_type = get_type(&val_type, module);

                // Check if the variable already exists
                if let Some((existing_var, existing_type)) =
                    self.variables.get(&assignment_stmt.target_name)
                {
                    let existing_type = get_type(existing_type, module);
                    if existing_type == val_ir_type {
                        // If the types are the same, reuse the existing variable
                        self.builder.def_var(*existing_var, val);
                    } else {
                        // If the types are different, create a new variable
                        let new_var = self.new_var();

                        self.variables
                            .insert(assignment_stmt.target_name.clone(), (new_var, val_type));
                        self.builder.declare_var(new_var, val_ir_type);
                        self.builder.def_var(new_var, val);
                    }
                }
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
                self.vec_as_array(vals, module.target_config().pointer_type(), module)
            }
        }
    }

    /// Generates code for an expression and returns the resulting IR value and its type.
    pub fn codegen_expr(
        &mut self,
        expr: &Expression,
        module: &JITModule,
    ) -> (ir::Value, knodiq_engine::Type) {
        match expr {
            Expression::IntLiteral(lit) => (
                self.builder.ins().iconst(TYPE_INT, *lit as i64),
                knodiq_engine::Type::Int,
            ),
            Expression::FloatLiteral(lit) => (
                self.builder.ins().f32const(*lit),
                knodiq_engine::Type::Float,
            ),
            Expression::Identifier(id) => {
                let (var, var_type) = self.variables.get(id).expect("Variable not found");
                (self.builder.use_var(*var), var_type.clone())
            }
            Expression::BinaryOp {
                op,
                left,
                right,
                left_type: _,
                right_type: _,
            } => {
                let (left_val, left_type) = self.codegen_expr(left, module);
                let (right_val, right_type) = self.codegen_expr(right, module);
                let op_type = type_of(&left_type, &right_type);
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
                let mut args = Vec::new();
                let mut arg_types = Vec::new();
                for arg in arguments {
                    let processed_expr = self.codegen_expr(arg, module);
                    args.push(processed_expr.0);
                    arg_types.push(processed_expr.1);
                }

                self.codegen_broadcast_op(
                    |vals, _types, slf, _module| slf.call_function(name, vals)[0],
                    args,
                    arg_types,
                    module,
                )
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
                self.codegen_broadcast_op(
                    |vals, types, slf, module| {
                        slf.codegen_op(
                            op,
                            vals[0],
                            vals[1],
                            types[0].clone(),
                            types[1].clone(),
                            module,
                        )
                    },
                    vec![left, right],
                    vec![left_type, right_type],
                    module,
                )
                .0
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
            Operator::Modulo => self.call_function("mod", vec![left, right])[0],
        }
    }

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
        let max_depth = val_types
            .iter()
            .max_by(|&a, &b| a.get_depth().cmp(&b.get_depth()))
            .unwrap()
            .get_depth();

        let mut resized_vals = Vec::new();
        let mut resized_val_types = Vec::new();
        for i in 0..vals.len() {
            let mut val = vals[i];
            let mut val_type = val_types[i].clone();

            while val_type.get_depth() < max_depth {
                // Resize the value to match the maximum depth
                let vec = vec![val];
                val = self.vec_as_array(vec, get_type(&val_type, module), module);
                val_type = knodiq_engine::Type::Array(Box::new(val_type.clone()));
            }

            resized_vals.push(val);
            resized_val_types.push(val_type);
        }

        self.recurse_op_broadcast(&mut op, resized_vals, resized_val_types, module)
    }

    fn recurse_op_broadcast<F>(
        &mut self,
        op: &mut F,
        args: Vec<ir::Value>,
        arg_types: Vec<knodiq_engine::Type>,
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
        if args.len() == 0 {
            return (
                self.builder.ins().iconst(TYPE_INT, 0),
                knodiq_engine::Type::Int,
            );
        }

        let is_all_array = arg_types
            .iter()
            .all(|ty| matches!(ty, knodiq_engine::Type::Array(_)));

        if is_all_array {
            let mut inside_types = Vec::new();

            for i in 0..args.len() {
                let val_type = &arg_types[i];

                match val_type {
                    knodiq_engine::Type::Array(inside_type) => {
                        inside_types.push(*inside_type.clone())
                    }
                    _ => unreachable!("Expected all types to be arrays"),
                }
            }

            let max_len = args.iter().map(|v| self.load_arr_size(*v)).max().unwrap();

            let mut operated_vals = Vec::new();
            let mut operated_types = Vec::new();
            self.codegen_loop(max_len, |slf, arr_idx| {
                let mut inside_vals = Vec::new();
                for arg_idx in 0..args.len() {
                    let arr_size = slf.load_arr_size(args[arg_idx]);
                    let is_arr_enough = slf.builder.ins().icmp(
                        ir::condcodes::IntCC::SignedLessThan,
                        arr_idx,
                        arr_size,
                    );

                    slf.codegen_if(
                        is_arr_enough,
                        Box::new(|slf: &mut Self| {
                            vec![slf.load_arr_val_at(
                                get_type(&inside_types[arg_idx], module),
                                args[arg_idx],
                                arr_idx,
                            )]
                        }),
                        |slf: &mut Self| {
                            let zero = slf.builder.ins().iconst(TYPE_INT, 0);
                            let is_arr_empty =
                                slf.builder
                                    .ins()
                                    .icmp(ir::condcodes::IntCC::Equal, arr_size, zero);
                            slf.codegen_if(
                                is_arr_empty,
                                |slf: &mut Self| {
                                    vec![
                                        slf.builder
                                            .ins()
                                            .iconst(get_type(&inside_types[arg_idx], module), 0),
                                    ]
                                },
                                |slf: &mut Self| {
                                    let arr_len = slf.load_arr_size(args[arg_idx]);
                                    let one = slf.builder.ins().iconst(TYPE_INT, 1);
                                    let last_idx = slf.builder.ins().isub(arr_len, one);
                                    vec![slf.load_arr_val_at(
                                        get_type(&inside_types[arg_idx], module),
                                        args[arg_idx],
                                        last_idx,
                                    )]
                                },
                            );
                            let current_block = slf.builder.current_block().unwrap();
                            let return_vals = slf.builder.block_params(current_block)[0];
                            vec![return_vals]
                        },
                    );

                    let current_block = slf.builder.current_block().unwrap();
                    let inside_val = slf.builder.block_params(current_block)[0];

                    inside_vals.push(inside_val);
                }

                let operated =
                    slf.recurse_op_broadcast(op, inside_vals, inside_types.clone(), module);
                operated_vals.push(operated.0);
                operated_types.push(operated.1);
            });

            (
                self.vec_as_array(operated_vals, get_type(&operated_types[0], module), module),
                knodiq_engine::Type::Array(Box::new(inside_types[0].clone())),
            )
        } else {
            let return_type = arg_types[0].clone();
            (op(args, arg_types, self, module), return_type)
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
    pub fn vec_as_array(
        &mut self,
        elems: Vec<ir::Value>,
        elem_type: ir::Type,
        module: &JITModule,
    ) -> ir::Value {
        let ptr_type = module.target_config().pointer_type();
        let arr_ptr = self.builder.ins().get_stack_pointer(ptr_type);

        let size_val = self.builder.ins().iconst(TYPE_INT, elems.len() as i64);
        self.builder
            .ins()
            .store(ir::MemFlags::new(), size_val, arr_ptr, 0);

        let data_offset = TYPE_INT.bytes() as i32;
        for (i, elem) in elems.iter().enumerate() {
            self.builder.ins().store(
                ir::MemFlags::new(),
                *elem,
                arr_ptr,
                data_offset + i as i32 * elem_type.bytes() as i32,
            );
        }
        arr_ptr
    }

    /// Iterates over an array at the given address, jumping to the provided block for each element.
    pub fn iter_arr<F>(&mut self, addr: ir::Value, val_type: ir::Type, iter_fn: F)
    where
        F: FnOnce(&mut Self, ir::Value, ir::Type),
    {
        let current_block = self.builder.current_block().unwrap();

        let data_bytes = self.builder.ins().iconst(val_type, val_type.bytes() as i64);

        // Get the size of the array
        let size = self
            .builder
            .ins()
            .load(TYPE_INT, ir::MemFlags::new(), addr, 0);

        // Calculate the starting address of the data by adding the size of the size
        let data_offset = self.builder.ins().iconst(TYPE_INT, TYPE_INT.bytes() as i64);

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

        // Calculate the address of the current elementl
        let offset_bytes = self.builder.ins().imul(i, data_bytes);
        let data_addr = self.builder.ins().iadd(addr, offset_bytes);
        let target_addr = self.builder.ins().iadd(data_addr, data_offset);

        // Load the value from the array
        let current_val = self
            .builder
            .ins()
            .load(val_type, ir::MemFlags::new(), target_addr, 0);
        iter_fn(self, current_val, val_type);

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

    pub fn load_arr_size(&mut self, addr: ir::Value) -> ir::Value {
        self.builder
            .ins()
            .load(TYPE_INT, ir::MemFlags::new(), addr, 0)
    }

    /// Generates code for a loop that iterates `count` times, executing the body for each iteration.
    pub fn codegen_loop<F>(&mut self, count: ir::Value, mut body: F) -> Block
    where
        F: FnMut(&mut Self, ir::Value),
    {
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
        body(self, i);

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

    pub fn codegen_if<ThenFn, ElseFn>(
        &mut self,
        condition: ir::Value,
        mut then_fn: ThenFn,
        mut else_fn: ElseFn,
    ) where
        ThenFn: FnMut(&mut Self) -> Vec<ir::Value>,
        ElseFn: FnMut(&mut Self) -> Vec<ir::Value>,
    {
        let current_block = self.builder.current_block().unwrap();
        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();

        self.builder
            .ins()
            .brif(condition, then_block, &[], else_block, &[]);

        self.builder.switch_to_block(then_block);
        let then_res = then_fn(self);
        self.builder.ins().jump(
            current_block,
            &then_res
                .iter()
                .map(|v| BlockArg::Value(*v))
                .collect::<Vec<_>>(),
        );
        self.builder.seal_block(then_block);

        self.builder.switch_to_block(else_block);
        let else_res = else_fn(self);
        self.builder.ins().jump(
            current_block,
            &else_res
                .iter()
                .map(|v| BlockArg::Value(*v))
                .collect::<Vec<_>>(),
        );
        self.builder.seal_block(else_block);

        self.builder.switch_to_block(current_block);
    }

    pub fn new_var(&mut self) -> Variable {
        let var = Variable::new(self.var_counter);
        self.var_counter += 1;
        var
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
            TYPE_FLOAT => TYPE_FLOAT,
            types::INVALID => left,
            _ => right,
        },
        TYPE_FLOAT => match right {
            TYPE_INT => TYPE_FLOAT,
            TYPE_FLOAT => TYPE_FLOAT,
            types::INVALID => left,
            _ => right,
        },
        types::INVALID => right,
        _ => left,
    }
}
