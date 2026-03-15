//
// © 2025-2026 Shuntaro Kasatani
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ErrorKind {
    /// Title: TopLevelStructField
    /// Payload: The name of the struct field that is defined in the top level
    /// Struct field is defined in the top level though it should be defined in a struct.
    TopLevelStructField,

    /// Title: ExprEndsWithDot
    /// Payload: None
    /// The expression ends with a dot.
    ExprEndsWithDot,

    /// Title: NonMemberTokenAfterDot
    /// Payload: None
    /// A token which is not a struct field or a function is after dot.
    NonMemberTokenAfterDot,

    /// Title: ArgForStructInit
    /// Payload: None
    /// Arguments are passed to the struct initializer.
    ArgForStructInit,

    /// Title: ExprBeginsWithDot
    /// Payload: None
    /// The expression begins with a dot.
    ExprBeginsWithDot,

    /// Title: InvalidLValue
    /// Payload: None
    /// The lvalue is expected, but it has an invalid token.
    InvalidLValue,

    /// Title: VarNotFound
    /// Payload: The name of the variable that is not found
    /// Variable is not found.
    VarNotFound,

    /// Title: FuncNotFound
    /// Payload: The name of the function that is not found
    /// Function is not found.
    FuncNotFound,

    /// Title: PrefixOpNotFound
    /// Payload: The symbol of the prefix operator that is not found
    /// Prefix operator is not found.
    PrefixOpNotFound,

    /// Title: InfixOrPostfixOpNotFound
    /// Payload: The symbol of the infix or postfix operator that is not found
    /// Infix or postfix operator is not found.
    InfixOrPostfixOpNotFound,

    /// Title: OpNotAssociative
    /// Payload: The symbol of the operator that is not associative
    /// Non-associative operator is used consecutively.
    OpNotAssociative,

    /// Title: NoReturnFuncInExpr
    /// Payload: The name of the function that has no return type
    /// Function without return type is used in an expression.
    NoReturnFuncInExpr,

    /// Title: MemberAccessOnPrimitive
    /// Payload: None
    /// Member access expression on a primitive type.
    MemberAccessOnPrimitive,

    /// Title: MemberFieldNotFound
    /// Payload: The name of the struct and the name of the member that is not found
    /// Member field of the struct is not found.
    MemberFieldNotFound,

    /// Title: MemberFuncNotFound
    /// Payload: The name of the struct and the name of the member that is not found
    /// Member function of the struct is not found.
    MemberFuncNotFound,

    /// Title: ArgOrderIncorrect
    /// Payload: The name of the function and the name of the argument that is out of order
    /// Argument order is incorrect.
    ArgOrderIncorrect,

    /// Title: DuplicateArg
    /// Payload: The name of the function and the name of the duplicate argument
    /// The same argument is given more than once.
    DuplicateArg,

    /// Title: ExtraArg
    /// Payload: The name of the function
    /// Too many arguments are given.
    ExtraArg,

    /// Title: MissingArg
    /// Payload: The name of the function
    /// Not enough arguments are given.
    MissingArg,

    /// Title: MissingArgLabel
    /// Payload: The name of the function
    /// A label of the argument is missing, but the argument requires a label.
    MissingArgLabel,

    /// Title: ArgTypeMismatch
    /// Payload: The name of the argument
    /// A type of the argument is wrong.
    ArgTypeMismatch,

    /// Title: TypeAnnotationMismatch
    /// Payload: The type of the annotation and the type of the expression
    /// The type annotation does not match the type of the expression.
    TypeAnnotationMismatch,

    /// Title: InvalidStructStmt
    /// Payload: The kind of the statement that is invalid
    /// An unexpected statement was found in the struct body.
    InvalidStructStmt,

    /// Title: TypeNotFound
    /// Payload: The name of the type that is not found
    /// The type is not found in the type registry.
    TypeNotFound,

    /// Title: NoTypeAnnotationOrDefVal
    /// Payload: None
    /// Both type annotation and default value are missing for the parameter.
    NoTypeAnnotationOrDefVal,

    /// Title: GlobalFuncCannotBeStatic
    /// Payload: The name of the function
    /// `static` keyword is applied to a global function.
    GlobalFuncCannotBeStatic,

    /// Title: WrongParamCountForInfix
    /// Payload: The symbol of the operator and the number of parameters
    /// An infix operator definition has a wrong number of parameters.
    WrongParamCountForInfix,

    /// Title: WrongParamCountForPrefix
    /// Payload: The symbol of the operator and the number of parameters
    /// A prefix operator definition has a wrong number of parameters.
    WrongParamCountForPrefix,

    /// Title: WrongParamCountForPostfix
    /// Payload: The symbol of the operator and the number of parameters
    /// A postfix operator definition has a wrong number of parameters.
    WrongParamCountForPostfix,

    /// Title: DuplicateVarName
    /// Payload: The name of the variable
    /// A variable with the same name already exists in the current scope, or its parent scopes.
    DuplicateVarName,

    /// Title: DuplicateStructName
    /// Payload: The name of the struct
    /// A struct with the same name already exists in the current scope, or its parent scopes.
    DuplicateStructName,

    /// Title: ReservedStructName
    /// Payload: The name of the struct
    /// The name of the struct is reserved by the language.
    ReservedStructName,

    /// Title: DuplicateFuncName
    /// Payload: The name of the function
    /// A function with the same name already exists in the current scope, or its parent scopes.
    DuplicateFuncName,

    /// Title: FuncCallInLValue
    /// Payload: None
    /// A function call is part of an LValue expression.
    FuncCallInLValue,

    /// Title: AssignTypeMismatch
    /// Payload: The type of the target and the type of the assigned value
    /// The type of the value does not match the type of the target variable.
    AssignTypeMismatch,

    /// Title: NonBoolTypeForCondition
    /// Payload: The type of the condition expression
    /// The condition expression does not have a bool type.
    NonBoolTypeForCondition,

    /// Title: ReturnTypeMismatch
    /// Payload: The type of the return value and the expected return type
    /// The type of the return value does not match the expected return type.
    ReturnTypeMismatch,

    /// Title: ReturnValueForNoReturnFunc
    /// Payload: None
    /// A value is given to a return statement in a function that does not return a value.
    ReturnValueForNoReturnFunc,

    /// Title: ReturnWithoutValueForReturnFunc
    /// Payload: The return type of the function
    /// A return statement without a value is used in a function that returns a value.
    ReturnWithoutValueForReturnFunc,

    /// Title: MissingReturn
    /// Payload: The return type of the function
    /// A function requires a return value but return statement is missing
    MissingReturn,

    /// Title: RecursiveCall
    /// Payload: None
    /// A recursive function call is found.
    RecursiveCall,

    /// Title: ImmutableAssignment
    /// Payload: None
    /// An assignment to an immutable variable (const or input).
    ImmutableAssignment,

    /// Title: StructCycle
    /// Payload: Name of the struct which has a cyclic field
    /// A struct has a cyclic field, so the compiler cannot determine the size.
    StructCycle,

    /// Title: StaticFuncCallOnInstance
    /// Payload: Name of the function
    /// A function is static but called on an instance.
    StaticFuncCallOnInstance,

    /// Title: StaticCallOfInstanceFunc
    /// Payload: Name of the function
    /// A function is not static but called statically.
    StaticCallOfInstanceFunc,

    /// Title: StaticVarAccess
    /// Payload: None
    /// A static access on variable is attempted.
    StaticVarAccess,

    /// Title: BuiltinVarAccess
    /// Payload: None
    /// A static access on variable in the Builtin is attempted.
    BuiltinVarAccess,

    /// Title: BuiltinFuncNotFound
    /// Payload: The name of the function
    /// A builtin function not found.
    BuiltinFuncNotFound,

    /// Title: BuiltinArgTypeMismatch
    /// Payload: None
    /// The type of the argument passed to an builtin function is wrong.
    BuiltinArgTypeMismatch,

    /// Title: ImportNotFound
    /// Payload: The path of the import
    /// An import path is not found.
    ImportNotFound,

    /// Title: CyclicDependency
    /// Payload: The path of the import
    /// A import makes a cyclic dependency.
    CyclicDependency,

    /// Title: CompilerBug
    /// Payload: Error message
    CompilerBug,
}
