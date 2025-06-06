#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Buffer,
    Float,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    InputDeclaration(InputDeclarationStatement),
    OutputDeclaration(OutputDeclarationStatement),
    VariableDeclaration(VariableDeclarationStatement),
    Assignment(AssignmentStatement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputDeclarationStatement {
    pub name: String,
    pub data_type: Type,
    pub initial_value: Expression,
    pub range: Option<(f64, f64)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutputDeclarationStatement {
    pub name: String,
    pub data_type: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarationStatement {
    pub name: String,
    pub data_type: Type,
    pub initial_value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentStatement {
    pub target_name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BinaryOp {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Literal(f64),
    Identifier(String),
    // FunctionCall {
    //     name: String,
    //     arguments: Vec<Expression>,
    // },
}
