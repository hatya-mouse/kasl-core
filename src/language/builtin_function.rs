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

use crate::Type;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ArgumentTypeSpec {
    /// Argument must be this concrete base type.
    Concrete(Type),
    /// Argument can be any of these base types.
    /// The type checker will resolve it to one BaseType at the call site.
    Polymorphic(Vec<Type>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub argument_specs: Vec<ArgumentTypeSpec>,
}

pub fn built_in_functions() -> HashMap<String, Function> {
    let mut functions = HashMap::new();

    let p_float_buffer = ArgumentTypeSpec::Polymorphic(vec![Type::Float, Type::Buffer]);

    functions.insert(
        "sin".to_string(),
        Function {
            name: "sin".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "cos".to_string(),
        Function {
            name: "cos".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "tan".to_string(),
        Function {
            name: "tan".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "asin".to_string(),
        Function {
            name: "asin".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "acos".to_string(),
        Function {
            name: "acos".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "atan".to_string(),
        Function {
            name: "atan".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "abs".to_string(),
        Function {
            name: "abs".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "sgn".to_string(),
        Function {
            name: "sgn".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "min".to_string(),
        Function {
            name: "min".to_string(),
            argument_specs: vec![p_float_buffer.clone(); 2],
        },
    );

    functions.insert(
        "max".to_string(),
        Function {
            name: "max".to_string(),
            argument_specs: vec![p_float_buffer.clone(); 2],
        },
    );

    functions.insert(
        "clamp".to_string(),
        Function {
            name: "clamp".to_string(),
            argument_specs: vec![
                p_float_buffer.clone(),
                p_float_buffer.clone(),
                p_float_buffer.clone(),
            ],
        },
    );

    functions.insert(
        "pow".to_string(),
        Function {
            name: "pow".to_string(),
            argument_specs: vec![p_float_buffer.clone(); 2],
        },
    );

    functions.insert(
        "sqrt".to_string(),
        Function {
            name: "sqrt".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "log".to_string(),
        Function {
            name: "log".to_string(), // Natural logarithm
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "log2".to_string(),
        Function {
            name: "log2".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "log10".to_string(),
        Function {
            name: "log10".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "saw".to_string(),
        Function {
            name: "saw".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "tri".to_string(),
        Function {
            name: "tri".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "square".to_string(),
        Function {
            name: "square".to_string(),
            argument_specs: vec![p_float_buffer.clone()],
        },
    );

    functions.insert(
        "rand".to_string(),
        Function {
            name: "rand".to_string(),
            argument_specs: vec![],
        },
    );

    functions.insert(
        "mix".to_string(),
        Function {
            name: "mix".to_string(),
            argument_specs: vec![
                p_float_buffer.clone(),
                p_float_buffer.clone(),
                p_float_buffer.clone(),
            ],
        },
    );

    functions.insert(
        "lerp".to_string(),
        Function {
            name: "lerp".to_string(),
            argument_specs: vec![
                p_float_buffer.clone(),
                p_float_buffer.clone(),
                p_float_buffer.clone(),
            ],
        },
    );

    functions.insert(
        "pi".to_string(),
        Function {
            name: "pi".to_string(),
            argument_specs: vec![],
        },
    );

    functions.insert(
        "time".to_string(),
        Function {
            name: "time".to_string(),
            argument_specs: vec![],
        },
    );

    functions.insert(
        "sample_rate".to_string(),
        Function {
            name: "sample_rate".to_string(),
            argument_specs: vec![],
        },
    );

    functions
}
