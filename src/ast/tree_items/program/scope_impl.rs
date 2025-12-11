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

use crate::{Function, InputVar, OutputVar, Program, Scope, StateVar, TypeDef};

impl Scope for Program {
    fn register_func(&mut self, func: Function) -> Result<(), crate::ConstructorError> {
        self.funcs.push(func);
        Ok(())
    }

    fn get_func(&self, name: &str) -> Option<&Function> {
        self.funcs.iter().find(|f| f.name == name)
    }

    fn get_func_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.funcs.iter_mut().find(|f| f.name == name)
    }

    fn register_type_def(&mut self, type_def: TypeDef) -> Result<(), crate::ConstructorError> {
        self.types.push(type_def);
        Ok(())
    }

    fn get_type_def(&self, name: &str) -> Option<&TypeDef> {
        self.types.iter().find(|t| t.name == name)
    }

    fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef> {
        self.types.iter_mut().find(|t| t.name == name)
    }

    fn register_input(&mut self, input: InputVar) -> Result<(), crate::ConstructorError> {
        self.inputs.push(input);
        Ok(())
    }

    fn get_input(&self, name: &str) -> Option<&InputVar> {
        self.inputs.iter().find(|i| i.name == name)
    }

    fn get_input_mut(&mut self, name: &str) -> Option<&mut InputVar> {
        self.inputs.iter_mut().find(|i| i.name == name)
    }

    fn register_output(&mut self, output: OutputVar) -> Result<(), crate::ConstructorError> {
        self.outputs.push(output);
        Ok(())
    }

    fn get_output(&self, name: &str) -> Option<&OutputVar> {
        self.outputs.iter().find(|o| o.name == name)
    }

    fn get_output_mut(&mut self, name: &str) -> Option<&mut OutputVar> {
        self.outputs.iter_mut().find(|o| o.name == name)
    }

    fn register_state(&mut self, state: StateVar) -> Result<(), crate::ConstructorError> {
        self.states.push(state);
        Ok(())
    }

    fn get_state(&self, name: &str) -> Option<&StateVar> {
        self.states.iter().find(|s| s.name == name)
    }

    fn get_state_mut(&mut self, name: &str) -> Option<&mut StateVar> {
        self.states.iter_mut().find(|s| s.name == name)
    }
}
