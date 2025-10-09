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

use crate::{
    Function, InputVar, OutputVar, ParserSymbolPathComponent, ResolverError, ResolverErrorType,
    StateVar, SymbolPath, SymbolPathComponent, TypeDef,
};

pub struct Program {
    pub main_func: Option<Function>,
    pub funcs: Vec<Function>,
    pub types: Vec<TypeDef>,
    pub states: Vec<StateVar>,
    pub inputs: Vec<InputVar>,
    pub outputs: Vec<OutputVar>,
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
        }
    }

    pub fn resolve_type(
        &self,
        type_path: &[ParserSymbolPathComponent],
    ) -> Result<SymbolPath, ResolverError> {
        let mut symbol_path = Vec::new();
        let mut current_scope = None;

        for (i, segment) in type_path.iter().enumerate() {
            if i == 0 {
                if let Some(type_def) = self.find_type_def(&segment.symbol) {
                    symbol_path.push(SymbolPathComponent::TypeDef(segment.symbol.clone()));
                    current_scope = Some(type_def);
                } else {
                    match segment.symbol.as_str() {
                        "CompInt" => symbol_path.push(SymbolPathComponent::CompInt),
                        "CompFloat" => symbol_path.push(SymbolPathComponent::CompFloat),
                        "CompBool" => symbol_path.push(SymbolPathComponent::CompBool),
                        _ => {
                            return Err(ResolverError {
                                error_type: ResolverErrorType::TypeNotFound(segment.symbol.clone()),
                                position: segment.range,
                            });
                        }
                    }
                }
            } else if let Some(some_scope) = current_scope {
                if let Some(type_def) = some_scope.find_type_def(&segment.symbol) {
                    symbol_path.push(SymbolPathComponent::TypeDef(segment.symbol.clone()));
                    current_scope = Some(type_def);
                } else {
                    return Err(ResolverError {
                        error_type: ResolverErrorType::TypeNotFound(segment.symbol.clone()),
                        position: segment.range,
                    });
                }
            }
        }

        Ok(symbol_path)
    }

    // -- Getter Functions --

    // # Function

    pub fn find_func(&self, name: &str) -> Option<&Function> {
        self.funcs.iter().find(|f| f.name == name)
    }

    pub fn find_func_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.funcs.iter_mut().find(|f| f.name == name)
    }

    // # Type

    pub fn find_type_def(&self, name: &str) -> Option<&TypeDef> {
        self.types.iter().find(|s| s.name == name)
    }

    pub fn find_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef> {
        self.types.iter_mut().find(|s| s.name == name)
    }

    // # State

    pub fn find_state_var(&self, name: &str) -> Option<&StateVar> {
        self.states.iter().find(|s| s.name == name)
    }

    pub fn find_state_var_mut(&mut self, name: &str) -> Option<&mut StateVar> {
        self.states.iter_mut().find(|s| s.name == name)
    }

    // # Input

    pub fn find_input(&self, name: &str) -> Option<&InputVar> {
        self.inputs.iter().find(|i| i.name == name)
    }

    pub fn find_input_mut(&mut self, name: &str) -> Option<&mut InputVar> {
        self.inputs.iter_mut().find(|i| i.name == name)
    }

    // # Output

    pub fn find_output(&self, name: &str) -> Option<&OutputVar> {
        self.outputs.iter().find(|o| o.name == name)
    }

    pub fn find_output_mut(&mut self, name: &str) -> Option<&mut OutputVar> {
        self.outputs.iter_mut().find(|o| o.name == name)
    }
}
