#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Payload {
    None,
    Str(String),
    StrPair(String, String),
    StrTriple(String, String, String),
    Num(usize),
    StrAndNum(String, usize),
    StrVec(Vec<String>),
}

impl Payload {
    pub fn to_vec(&self) -> Vec<String> {
        match self {
            Payload::None => vec![],
            Payload::Str(a) => vec![a.clone()],
            Payload::StrPair(a, b) => vec![a.clone(), b.clone()],
            Payload::StrTriple(a, b, c) => vec![a.clone(), b.clone(), c.clone()],
            Payload::Num(a) => vec![a.to_string()],
            Payload::StrAndNum(a, b) => vec![a.clone(), b.to_string()],
            Payload::StrVec(a) => a.clone(),
        }
    }
}
