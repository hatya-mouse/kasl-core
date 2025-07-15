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

use crate::{Expression, Operator, Statement, SymbolInfo};
use cranelift_codegen::{
    entity::EntityRef,
    ir::{self, Block, InstBuilder, types},
};
use cranelift_frontend::{FunctionBuilder, Variable};
use cranelift_jit::JITModule;
use cranelift_module::Module;
use std::collections::HashMap;

const TYPE_INT: ir::Type = types::I32;
const TYPE_FLOAT: ir::Type = types::F32;

pub struct Translator<'a> {
    pub builder: FunctionBuilder<'a>,
    pub variables: HashMap<String, (Variable, ir::Type)>,
    pub return_vars: Vec<String>,
    functions: HashMap<String, ir::FuncRef>,
    entry_block: Block,
    input_ptr: ir::Value,
    output_ptr: ir::Value,
    input_count: ir::Value,
    output_count: ir::Value,
}

impl<'a> Translator<'a> {
    pub fn new(
        builder: FunctionBuilder<'a>,
        functions: HashMap<String, ir::FuncRef>,
        entry_block: Block,
    ) -> Self {
        Translator {
            builder,
            variables: HashMap::new(),
            return_vars: Vec::new(),
            functions,
            entry_block,
            input_ptr: ir::Value::new(0),
            output_ptr: ir::Value::new(0),
            input_count: ir::Value::new(0),
            output_count: ir::Value::new(0),
        }
    }

    pub fn setup_array_interface(
        &mut self,
        input_names: &[String],
        output_names: &[String],
        inputs: &HashMap<String, SymbolInfo>,
        outputs: &HashMap<String, SymbolInfo>,
        module: &JITModule,
    ) {
        let params = self.builder.block_params(self.entry_block);
        self.input_ptr = params[0];
        self.input_count = params[1];
        self.output_ptr = params[2];
        self.output_count = params[3];

        let mut input_offset = 0;
        for (_, input_name) in input_names.iter().enumerate() {
            if let Some(input_info) = inputs.get(input_name) {
                let var = Variable::new(self.variables.len());
                let val_type = get_type(&input_info.value_type, module);

                let offset = self.builder.ins().iconst(types::I32, input_offset);
                let addr = self.builder.ins().iadd(self.input_ptr, offset);
                let val = self
                    .builder
                    .ins()
                    .load(val_type, ir::MemFlags::new(), addr, 0);

                self.variables.insert(input_name.clone(), (var, val_type));
                self.builder.declare_var(var, val_type);
                self.builder.def_var(var, val);

                // Calculate the offset for the next element
                input_offset += match val_type {
                    types::I32 => 4,
                    types::F32 => 4,
                    _ => 8,
                }
            }
        }

        for output_name in output_names.iter() {
            if let Some(output_info) = outputs.get(output_name) {
                let var = Variable::new(self.variables.len());
                let val_type = get_type(&output_info.value_type, module);

                self.variables.insert(output_name.clone(), (var, val_type));
                self.return_vars.push(output_name.clone());
                self.builder.declare_var(var, val_type);

                let default_val = match val_type {
                    types::I32 => self.builder.ins().iconst(types::I32, 0),
                    types::F32 => self.builder.ins().f32const(0.0),
                    _ => self.builder.ins().iconst(val_type, 0),
                };

                self.builder.def_var(var, default_val);
            }
        }
    }

    pub fn finalize_array_interface(&mut self, output_names: &[String]) {
        let mut output_offset = 0;
        for output_name in output_names.iter() {
            if let Some((var, val_type)) = self.variables.get(output_name) {
                let val = self.builder.use_var(*var);
                let offset = self.builder.ins().iconst(types::I32, output_offset);
                let addr = self.builder.ins().iadd(self.output_ptr, offset);
                self.builder.ins().store(ir::MemFlags::new(), val, addr, 0);

                // Calculate the offset for the next element
                output_offset += match val_type {
                    &types::I32 => 4,
                    &types::F32 => 4,
                    _ => 8,
                };
            }
        }

        self.builder.ins().return_(&[]);
    }

    pub fn codegen_stmt(&mut self, statement: &Statement, pointer_type: ir::Type) {
        match statement {
            Statement::VariableDeclaration(var_decl) => {
                let var = Variable::new(self.variables.len());
                let (val, val_type) = self.codegen_expr(&var_decl.initial_value, pointer_type);

                self.variables
                    .insert(var_decl.name.clone(), (var, val_type));

                self.builder.declare_var(var, val_type);
                self.builder.def_var(var, val);
            }
            Statement::Assignment(assignment_stmt) => {
                if let Some((var, _)) = self.variables.get(&assignment_stmt.target_name).cloned() {
                    let (val, _) = self.codegen_expr(&assignment_stmt.value, pointer_type);
                    self.builder.def_var(var, val);
                }
            }
            Statement::ForLoop(_for_loop_stmt) => {
                todo!()
            }
            _ => return,
        }
    }

    pub fn value_as_ir(&mut self, value: &knodiq_engine::Value, module: &JITModule) -> ir::Value {
        match value {
            knodiq_engine::Value::Int(i) => self.builder.ins().iconst(TYPE_INT, *i as i64),
            knodiq_engine::Value::Float(f) => self.builder.ins().f32const(*f),
            knodiq_engine::Value::Array(arr) => {
                let arr_type = module.target_config().pointer_type();
                let arr_ptr = self.builder.ins().get_stack_pointer(arr_type);
                for (i, elem) in arr.iter().enumerate() {
                    let elem_val = self.value_as_ir(elem, module);
                    self.builder
                        .ins()
                        .store(ir::MemFlags::new(), elem_val, arr_ptr, i as i32);
                }
                arr_ptr
            }
        }
    }

    pub fn codegen_expr(
        &mut self,
        expr: &Expression,
        pointer_type: ir::Type,
    ) -> (ir::Value, ir::Type) {
        match expr {
            Expression::IntLiteral(lit) => {
                (self.builder.ins().iconst(types::I32, *lit as i64), TYPE_INT)
            }
            Expression::FloatLiteral(lit) => {
                let val = self.builder.ins().f32const(*lit);
                println!("Float literal {}: {:?}", lit, val);
                (val, TYPE_FLOAT)
            }
            Expression::Identifier(id) => {
                let (var, var_type) = self.variables.get(id).expect("Variable not found");
                (self.builder.use_var(*var), *var_type)
            }
            Expression::BinaryOp { op, left, right } => {
                let (left_val, left_type) = self.codegen_expr(left, pointer_type);
                let (right_val, right_type) = self.codegen_expr(right, pointer_type);
                let op_type = eval_type(left_type, right_type);
                (
                    self.codegen_op(op, left_val, right_val, op_type, pointer_type),
                    op_type,
                )
            }
            Expression::FunctionCall { name, arguments } => {
                let args = arguments
                    .iter()
                    .map(|arg| self.codegen_expr(arg, pointer_type))
                    .collect::<Vec<(ir::Value, ir::Type)>>();
                let arg_val = args.iter().map(|(val, _)| *val).collect::<Vec<ir::Value>>();
                let mut inferred_type = types::INVALID;

                for (_, val_type) in args.iter() {
                    inferred_type = eval_type(inferred_type, *val_type);
                }

                if let Some(function) = self.functions.get(name) {
                    let inst = self.builder.ins().call(*function, &arg_val);
                    (
                        self.builder
                            .inst_results(inst)
                            .first()
                            .unwrap_or(&ir::Value::new(0))
                            .clone(),
                        inferred_type,
                    )
                } else {
                    (ir::Value::new(0), TYPE_INT)
                }
            }
        }
    }

    pub fn codegen_op(
        &mut self,
        op: &Operator,
        left: ir::Value,
        right: ir::Value,
        op_type: ir::Type,
        _pointer_type: ir::Type,
    ) -> ir::Value {
        match op_type {
            TYPE_INT => self.codegen_int_op(op, left, right),
            TYPE_FLOAT => self.codegen_float_op(op, left, right),
            // pointer_type => self.codegen_pointer_op(op, left, right, pointer_type),
            _ => panic!("Unsupported operation type: {:?}", op_type),
        }
    }

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
            _ => panic!("Unsupported float operation: {:?}", op),
        }
    }

    pub fn get_returns(&mut self) -> Vec<ir::Value> {
        let return_vals = self
            .return_vars
            .iter()
            .map(|var| self.builder.use_var(self.variables[var].0))
            .collect::<Vec<ir::Value>>();
        self.builder.ins().return_(&return_vals);
        return_vals
    }
}

pub fn get_type(value_type: &knodiq_engine::Type, module: &JITModule) -> types::Type {
    match value_type {
        knodiq_engine::Type::Int => TYPE_INT,
        knodiq_engine::Type::Float => TYPE_FLOAT,
        knodiq_engine::Type::Array(_) => module.target_config().pointer_type(),
        knodiq_engine::Type::None => types::INVALID,
    }
}

pub fn eval_type(left: ir::Type, right: ir::Type) -> ir::Type {
    match left {
        types::I32 => match right {
            types::I32 => types::I32,
            types::F32 => types::F32,
            types::INVALID => right,
            _ => right,
        },
        types::F32 => match right {
            types::I32 => types::F32,
            types::F32 => types::F32,
            types::INVALID => left,
            _ => right,
        },
        types::INVALID => right,
        _ => left,
    }
}
