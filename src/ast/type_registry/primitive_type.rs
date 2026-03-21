use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum PrimitiveType {
    Int,
    Float,
    Bool,
    Void,
}

impl PrimitiveType {
    pub fn size(&self) -> usize {
        match self {
            PrimitiveType::Bool => 1,
            PrimitiveType::Int => 4,
            PrimitiveType::Float => 4,
            PrimitiveType::Void => 0,
        }
    }

    pub fn alignment(&self) -> u8 {
        match self {
            PrimitiveType::Bool => 1,
            PrimitiveType::Int => 4,
            PrimitiveType::Float => 4,
            PrimitiveType::Void => 1,
        }
    }
}

impl FromStr for PrimitiveType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Int" => Ok(PrimitiveType::Int),
            "Float" => Ok(PrimitiveType::Float),
            "Bool" => Ok(PrimitiveType::Bool),
            _ => Err(()),
        }
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveType::Bool => write!(f, "Bool"),
            PrimitiveType::Float => write!(f, "Bool"),
            PrimitiveType::Int => write!(f, "Int"),
            PrimitiveType::Void => write!(f, "Void"),
        }
    }
}
