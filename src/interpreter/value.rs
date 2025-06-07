#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Float(f32),
    Buffer(Vec<f32>),
}
