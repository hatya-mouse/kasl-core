pub mod compilation_data;
pub mod data;
pub mod namespace_registry;
pub mod scope_manager;
pub mod symbol_table;
pub mod type_registry;

pub use compilation_data::CompilationData;
pub use data::Range;
pub use namespace_registry::{
    FunctionID, NameSpace, NameSpaceID, OperatorID, ParserStmtID, StructID, SymbolPath,
    SymbolPathComponent, VariableID,
};
pub use scope_manager::{InputAttribute, Scope, ScopeID, ScopeRegistry, ScopeVar};
pub use symbol_table::{
    Expr, ExprKind, FuncCallArg, FuncParam, Function, IfArm, InfixOperator,
    InfixOperatorProperties, OperatorAssociativity, OperatorContext, PostfixOperator,
    PostfixOperatorProperties, PrefixOperator, PrefixOperatorProperties, Statement,
};
