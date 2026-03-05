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
    Function, InfixOperator, InfixOperatorProperties, InputVar, OutputVar, PrefixOperator, Program,
    ScopeVar, StateVar, StructDecl, SymbolPath,
    data::{SymbolID, SymbolMetadata},
    symbol_path,
};

impl Program {
    // -- Function --
    pub fn register_func(&mut self, func: Function, path: SymbolPath) {
        let id = self.get_next_id();
        self.register_id(path, id);
        self.add_metadata(id, SymbolMetadata::new(func.return_type));
        self.funcs.insert(id, func);
    }

    pub fn get_func(&self, id: &SymbolID) -> Option<&Function> {
        self.funcs.get(id)
    }

    pub fn get_func_mut(&mut self, id: &SymbolID) -> Option<&mut Function> {
        self.funcs.get_mut(id)
    }

    // -- StructDecl --
    pub fn register_struct_decl(&mut self, struct_decl: StructDecl, path: SymbolPath) {
        let id = self.get_next_id();
        self.register_id(path, id);
        self.add_metadata(id, SymbolMetadata::no_type());
        self.structs.insert(id, struct_decl);
    }

    pub fn get_struct_decl(&self, id: &SymbolID) -> Option<&StructDecl> {
        self.structs.get(id)
    }

    pub fn get_struct_decl_mut(&mut self, id: &SymbolID) -> Option<&mut StructDecl> {
        self.structs.get_mut(id)
    }

    // -- Input --
    pub fn register_input(&mut self, input: InputVar) {
        let id = self.get_next_id();
        self.register_id(symbol_path![input.name.clone()], id);
        self.add_metadata(id, SymbolMetadata::with_type(input.value_type));
        self.inputs.insert(id, input);
    }

    pub fn get_input(&self, id: &SymbolID) -> Option<&InputVar> {
        self.inputs.get(id)
    }

    pub fn get_input_mut(&mut self, id: &SymbolID) -> Option<&mut InputVar> {
        self.inputs.get_mut(id)
    }

    // -- Output --
    pub fn register_output(&mut self, output: OutputVar) {
        let id = self.get_next_id();
        self.register_id(symbol_path![output.name.clone()], id);
        self.add_metadata(id, SymbolMetadata::with_type(output.value_type));
        self.outputs.insert(id, output);
    }

    pub fn get_output(&self, id: &SymbolID) -> Option<&OutputVar> {
        self.outputs.get(id)
    }

    pub fn get_output_mut(&mut self, id: &SymbolID) -> Option<&mut OutputVar> {
        self.outputs.get_mut(id)
    }

    // -- StateVar --
    pub fn register_state(&mut self, state: StateVar) {
        let id = self.get_next_id();
        self.register_id(symbol_path![state.name.clone()], id);
        self.add_metadata(id, SymbolMetadata::with_type(state.value_type));
        self.states.insert(id, state);
    }

    pub fn get_state(&self, id: &SymbolID) -> Option<&StateVar> {
        self.states.get(id)
    }

    pub fn get_state_mut(&mut self, id: &SymbolID) -> Option<&mut StateVar> {
        self.states.get_mut(id)
    }

    // -- ScopeVar --
    pub fn register_scope_var(&mut self, scope_var: ScopeVar, path: SymbolPath) {
        let id = self.get_next_id();
        self.register_id(path, id);
        self.add_metadata(id, SymbolMetadata::with_type(scope_var.value_type));
        self.vars.insert(id, scope_var);
    }

    pub fn get_scope_var(&self, id: &SymbolID) -> Option<&ScopeVar> {
        self.vars.get(id)
    }

    pub fn get_scope_var_mut(&mut self, id: &SymbolID) -> Option<&mut ScopeVar> {
        self.vars.get_mut(id)
    }

    // -- InfixOperator --
    /// Register a new infix operator properties.
    pub fn register_infix_operator(&mut self, symbol: &str, properties: InfixOperatorProperties) {
        self.infix_operator_properties
            .insert(symbol.to_string(), properties);
    }

    /// Get an immutable reference to the InfixOperator properties by its symbol.
    pub fn get_infix_operator(&self, symbol: &str) -> Option<&InfixOperatorProperties> {
        self.infix_operator_properties.get(symbol)
    }

    /// Register an InfixOperator to the program
    pub fn register_infix_func(&mut self, operator: InfixOperator) {
        let id = self.get_next_id();
        self.register_id(symbol_path![operator.symbol.clone()], id);
        self.add_metadata(id, SymbolMetadata::with_type(operator.return_type));
        self.infix_operators.insert(id, operator);
    }

    /// Get an immutable reference to the InfixOperator by its SymbolPath and type of operand.
    pub fn get_infix_func_by_path(
        &self,
        path: &SymbolPath,
        lhs_type: &SymbolID,
        rhs_type: &SymbolID,
    ) -> Option<&InfixOperator> {
        let ids = self.get_id_by_path(path)?;
        let id = ids.iter().find(|id| {
            self.infix_operators
                .get(id)
                .is_some_and(|op| &op.lhs.value_type == lhs_type && &op.rhs.value_type == rhs_type)
        })?;
        self.infix_operators.get(id)
    }

    /// Get an immutable reference to the InfixOperator.
    pub fn get_infix_func(&self, id: &SymbolID) -> Option<&InfixOperator> {
        self.infix_operators.get(id)
    }

    /// Get a mutable reference to the InfixOperator.
    pub fn get_infix_func_mut(&mut self, id: &SymbolID) -> Option<&mut InfixOperator> {
        self.infix_operators.get_mut(id)
    }

    /// Register a PrefixOperator to the program
    pub fn register_prefix_func(&mut self, operator: PrefixOperator) {
        let id = self.get_next_id();
        self.register_id(symbol_path![operator.symbol.clone()], id);
        self.add_metadata(id, SymbolMetadata::with_type(operator.return_type));
        self.prefix_operators.insert(id, operator);
    }

    /// Get an immutable reference to the PrefixOperator by its SymbolPath and type of operand.
    pub fn get_prefix_func_by_path(
        &self,
        path: &SymbolPath,
        operand_type: &SymbolID,
    ) -> Option<&PrefixOperator> {
        let ids = self.get_id_by_path(path)?;
        let id = ids.iter().find(|id| {
            self.prefix_operators
                .get(id)
                .is_some_and(|op| &op.operand.value_type == operand_type)
        })?;
        self.prefix_operators.get(id)
    }

    /// Get an immutable reference to the PrefixOperator.
    pub fn get_prefix_func(&self, id: &SymbolID) -> Option<&PrefixOperator> {
        self.prefix_operators.get(id)
    }

    /// Get a mutable reference to the PrefixOperator.
    pub fn get_prefix_func_mut(&mut self, id: &SymbolID) -> Option<&mut PrefixOperator> {
        self.prefix_operators.get_mut(id)
    }
}
