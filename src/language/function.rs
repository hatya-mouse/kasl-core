use crate::Type;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Type>,
    pub return_type: Type,
}

pub fn built_in_functions() -> HashMap<String, Function> {
    let mut functions = HashMap::new();

    functions.insert(
        "sin".to_string(),
        Function {
            name: "sin".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "cos".to_string(),
        Function {
            name: "cos".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "tan".to_string(),
        Function {
            name: "tan".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "asin".to_string(),
        Function {
            name: "asin".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "acos".to_string(),
        Function {
            name: "acos".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "atan".to_string(),
        Function {
            name: "atan".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "abs".to_string(),
        Function {
            name: "abs".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "sgn".to_string(),
        Function {
            name: "sgn".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "min".to_string(),
        Function {
            name: "min".to_string(),
            arguments: vec![Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "max".to_string(),
        Function {
            name: "max".to_string(),
            arguments: vec![Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "clamp".to_string(),
        Function {
            name: "clamp".to_string(),
            arguments: vec![Type::Float, Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "pow".to_string(),
        Function {
            name: "pow".to_string(),
            arguments: vec![Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "sqrt".to_string(),
        Function {
            name: "sqrt".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "log".to_string(),
        Function {
            name: "log".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "log2".to_string(),
        Function {
            name: "log2".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "log10".to_string(),
        Function {
            name: "log10".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "saw".to_string(),
        Function {
            name: "saw".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "tri".to_string(),
        Function {
            name: "tri".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "square".to_string(),
        Function {
            name: "square".to_string(),
            arguments: vec![Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "rand".to_string(),
        Function {
            name: "rand".to_string(),
            arguments: vec![],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "mix".to_string(),
        Function {
            name: "mix".to_string(),
            arguments: vec![Type::Float, Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "lerp".to_string(),
        Function {
            name: "lerp".to_string(),
            arguments: vec![Type::Float, Type::Float, Type::Float],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "pi".to_string(),
        Function {
            name: "pi".to_string(),
            arguments: vec![],
            return_type: Type::Float,
        },
    );

    functions.insert(
        "time".to_string(),
        Function {
            name: "time".to_string(),
            arguments: vec![],
            return_type: Type::Buffer,
        },
    );

    functions.insert(
        "sample_rate".to_string(),
        Function {
            name: "sample_rate".to_string(),
            arguments: vec![],
            return_type: Type::Float,
        },
    );

    functions
}
