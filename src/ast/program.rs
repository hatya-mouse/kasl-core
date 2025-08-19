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
    Function, InputVar, OutputVar, ProtocolType, StateVar, StructType, type_def::Initializer,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
    pub inputs: Vec<InputVar>,
    pub outputs: Vec<OutputVar>,
    pub states: Vec<StateVar>,

    pub structs: Vec<StructType>,
    pub protocols: Vec<ProtocolType>,

    pub intliteral: Option<Initializer>,
    pub floatliteral: Option<Initializer>,
    pub boolliteral: Option<Initializer>,
}
