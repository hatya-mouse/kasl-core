//
// Copyright 2025-2026 Shuntaro Kasatani
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

use std::collections::HashMap;

use crate::{
    Function, InfixOperator, InfixOperatorProperties, InputVar, OutputVar, PrefixOperator,
    StateVar, SymbolPath, TypeDef,
};

#[derive(Debug)]
pub struct Program {
    pub main_func: Option<Function>,
    pub funcs: Vec<Function>,
    pub types: Vec<TypeDef>,
    pub states: Vec<StateVar>,
    pub inputs: Vec<InputVar>,
    pub outputs: Vec<OutputVar>,

    pub infix_operator_properties: HashMap<String, InfixOperatorProperties>,
    pub infix_operators: Vec<InfixOperator>,
    pub prefix_operator_symbols: Vec<String>,
    pub prefix_operators: Vec<PrefixOperator>,

    pub bool_literal_type: Option<SymbolPath>,
    pub int_literal_type: Option<SymbolPath>,
    pub float_literal_type: Option<SymbolPath>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            main_func: None,
            funcs: Vec::new(),
            types: Vec::new(),
            states: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),

            infix_operator_properties: HashMap::new(),
            infix_operators: Vec::new(),
            prefix_operator_symbols: Vec::new(),
            prefix_operators: Vec::new(),

            bool_literal_type: None,
            int_literal_type: None,
            float_literal_type: None,
        }
    }
}

impl Program {
    // -- TypeDef --
    pub fn get_type_def(&self, name: &str) -> Option<&TypeDef> {
        self.types.iter().find(|s| s.name == name)
    }

    pub fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef> {
        self.types.iter_mut().find(|s| s.name == name)
    }

    // -- Function --
    pub fn register_func(&mut self, func: Function) {
        self.funcs.push(func);
    }

    pub fn get_func(&self, name: &str) -> Option<&Function> {
        self.funcs.iter().find(|s| s.name == name)
    }

    pub fn get_func_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.funcs.iter_mut().find(|s| s.name == name)
    }

    // -- Input --
    pub fn register_input(&mut self, input: InputVar) {
        self.inputs.push(input);
    }

    pub fn get_input(&self, name: &str) -> Option<&InputVar> {
        self.inputs.iter().find(|s| s.name == name)
    }

    pub fn get_input_mut(&mut self, name: &str) -> Option<&mut InputVar> {
        self.inputs.iter_mut().find(|s| s.name == name)
    }

    // -- Output --
    pub fn register_output(&mut self, output: OutputVar) {
        self.outputs.push(output);
    }

    pub fn get_output(&self, name: &str) -> Option<&OutputVar> {
        self.outputs.iter().find(|s| s.name == name)
    }

    pub fn get_output_mut(&mut self, name: &str) -> Option<&mut OutputVar> {
        self.outputs.iter_mut().find(|s| s.name == name)
    }

    // -- State --
    pub fn register_state(&mut self, state: StateVar) {
        self.states.push(state);
    }

    pub fn get_state(&self, name: &str) -> Option<&StateVar> {
        self.states.iter().find(|s| s.name == name)
    }

    pub fn get_state_mut(&mut self, name: &str) -> Option<&mut StateVar> {
        self.states.iter_mut().find(|s| s.name == name)
    }

    // -- InfixOperator --
    /// Register a new infix operator properties.
    pub fn register_infix_properties(
        &mut self,
        symbol: String,
        properties: InfixOperatorProperties,
    ) {
        self.infix_operator_properties.insert(symbol, properties);
    }

    /// Get an immutable reference to the InfixOperator properties by its symbol.
    pub fn get_infix_properties(&self, symbol: &str) -> Option<&InfixOperatorProperties> {
        self.infix_operator_properties.get(symbol)
    }

    /// Register an InfixOperator to the program
    pub fn register_infix_func(&mut self, operator: InfixOperator) {
        self.infix_operators.push(operator);
    }

    /// Get an immutable reference to the InfixOperator by its path.
    pub fn get_infix_func(
        &self,
        lhs_type: &SymbolPath,
        rhs_type: &SymbolPath,
        operator_symbol: &str,
    ) -> Option<&InfixOperator> {
        self.infix_operators
            .iter()
            .filter(|op| op.symbol == operator_symbol)
            .find(|op| {
                if let (Some(lhs), Some(rhs)) = (&op.lhs, &op.rhs) {
                    if let (Some(lhs_value_type), Some(rhs_value_type)) =
                        (&lhs.value_type, &rhs.value_type)
                    {
                        return lhs_value_type == lhs_type && rhs_value_type == rhs_type;
                    }
                }
                false
            })
    }

    /// Get a mutable reference to the InfixOperator by its path.
    pub fn get_infix_func_mut(
        &mut self,
        lhs_type: &SymbolPath,
        rhs_type: &SymbolPath,
        operator_symbol: &str,
    ) -> Option<&mut InfixOperator> {
        self.infix_operators
            .iter_mut()
            .filter(|op| op.symbol == operator_symbol)
            .find(|op| {
                if let (Some(lhs), Some(rhs)) = (&op.lhs, &op.rhs) {
                    if let (Some(lhs_value_type), Some(rhs_value_type)) =
                        (&lhs.value_type, &rhs.value_type)
                    {
                        return lhs_value_type == lhs_type && rhs_value_type == rhs_type;
                    }
                }
                false
            })
    }

    // -- PrefixOperator --
    /// Register a new prefix operator.
    pub fn register_prefix_operator(&mut self, symbol: String) {
        self.prefix_operator_symbols.push(symbol);
    }

    /// Check if a prefix operator is registered.
    pub fn has_prefix_operator(&self, symbol: &String) -> bool {
        self.prefix_operator_symbols.contains(symbol)
    }

    /// Register a PrefixOperator to the program
    pub fn register_prefix_func(&mut self, operator: PrefixOperator) {
        self.prefix_operators.push(operator);
    }

    /// Get an immutable reference to the PrefixOperator by its path.
    pub fn get_prefix_func(
        &self,
        operand_type: &SymbolPath,
        operator_symbol: &str,
    ) -> Option<&PrefixOperator> {
        self.prefix_operators
            .iter()
            .filter(|op| op.symbol == operator_symbol)
            .find(|op| {
                if let Some(operand) = &op.operand {
                    if let Some(value_type) = &operand.value_type {
                        return value_type == operand_type;
                    }
                }
                false
            })
    }

    /// Get a mutable reference to the PrefixOperator by its path.
    pub fn get_prefix_func_mut(
        &mut self,
        operand_type: &SymbolPath,
        operator_symbol: &str,
    ) -> Option<&mut PrefixOperator> {
        self.prefix_operators
            .iter_mut()
            .filter(|op| op.symbol == operator_symbol)
            .find(|op| {
                if let Some(operand) = &op.operand {
                    if let Some(value_type) = &operand.value_type {
                        return value_type == operand_type;
                    }
                }
                false
            })
    }
}
