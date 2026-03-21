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

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, strum::Display)]
pub enum ErrorKind {
    /// Title: ParserError
    /// Payload: ParserError
    ParserError,

    /// Title: TopLevelVar
    /// Payload: The name of the variable declared with `var` keyword that is defined in the top level
    /// A `var` variable is defined in the top level.
    TopLevelVar,

    /// Title: NonMemberTokenAfterDot
    /// Payload: None
    /// A token which is not a struct field or a function is after dot.
    NonMemberTokenAfterDot,

    /// Title: ArgForStructInit
    /// Payload: None
    /// Arguments are passed to the struct initializer.
    ArgForStructInit,

    /// Title: ExprEndsWithDot
    /// Payload: None
    /// The expression ends with a dot.
    ExprEndsWithDot,

    /// Title: ExprBeginsWithInvalid
    /// Payload: None
    /// The expression begins with an invalid token.
    ExprBeginsWithInvalid,

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

    /// Title: PrefixOpNotDefined
    /// Payload: The symbol of the prefix operator that is not defined
    /// Prefix operator is not defined.
    PrefixOpNotDefined,

    /// Title: InfixOrPostfixOpNotDefined
    /// Payload: The symbol of the infix or postfix operator that is not defined
    /// Infix or postfix operator is not defined.
    InfixOrPostfixOpNotDefined,

    /// Title: InfixOpNotFound
    /// Payload: The symbol of the infix operator, the type of the left operand, and the type of the right operand
    /// Infix operator is not found for the given types.
    InfixOpNotFound,

    /// Title: PrefixOpNotFound
    /// Payload: The symbol of the prefix operator, and the type of the operand
    /// Prefix operator is not found for the given type.
    PrefixOpNotFound,

    /// Title: PostfixOpNotFound
    /// Payload: The symbol of the postfix operator, and the type of the operand
    /// Postfix operator is not found for the given type.
    PostfixOpNotFound,

    /// Title: OpNotAssociative
    /// Payload: The symbol of the operator that is not associative
    /// Non-associative operator is used consecutively.
    OpNotAssociative,

    /// Title: MemberAccessOnPrimitive
    /// Payload: None
    /// Member access expression on a primitive type.
    MemberAccessOnPrimitive,

    /// Title: MemberAccessOnArray
    /// Payload: The type of an array
    /// Member access expression on an array type.
    MemberAccessOnArray,

    /// Title: MemberFieldNotFound
    /// Payload: The name of the struct and the name of the member that is not found
    /// Member field of the struct is not found.
    MemberFieldNotFound,

    /// Title: MemberFuncNotFound
    /// Payload: The name of the struct and the name of the member that is not found
    /// Member function of the struct is not found.
    MemberFuncNotFound,

    /// Title: ArgOrderIncorrect
    /// Payload: The name of the argument that is out of order
    /// Argument order is incorrect.
    ArgOrderIncorrect,

    /// Title: DuplicateArg
    /// Payload: The name of the duplicate argument
    /// The same argument is given more than once.
    DuplicateArgIsGiven,

    /// Title: ExtraArg
    /// Payload: The number of arguments expected
    /// Too many arguments are given.
    ExtraArg,

    /// Title: MissingArg
    /// Payload: The name of the argument that is missing
    /// Not enough arguments are given.
    MissingArg,

    /// Title: MissingArgLabel
    /// Payload: The name and the label of the argument that is missing a label
    /// A label of the argument is missing, but the argument requires a label.
    MissingArgLabel,

    /// Title: ArgTypeMismatch
    /// Payload: The name of the argument, the expected type, and the actual type
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

    /// Title: DuplicateName
    /// Payload: The name that is already used
    /// A declaration with the same name already exists in the current scope, or its parent scopes.
    DuplicateName,

    /// Title: DuplicateInfixDefine
    /// Payload: The symbol of the operator definition
    /// An infix operator with the same symbol already exists.
    DuplicateInfixDefine,

    /// Title: DuplicatePrefixDefine
    /// Payload: The symbol of the operator definition
    /// An prefix operator with the same symbol already exists.
    DuplicatePrefixDefine,

    /// Title: DuplicatePostfixDefine
    /// Payload: The symbol of the operator definition
    /// An postfix operator with the same symbol already exists.
    DuplicatePostfixDefine,

    /// Title: DuplicateInfixFunc
    /// Payload: The symbol of the operator definition, lhs value type, rhs value type
    /// An operator function with the same symbol and the operand type is defined.
    DuplicateInfixFunc,

    /// Title: DuplicatePrefixFunc
    /// Payload: The symbol of the operator definition and operand value type
    /// An operator function with the same symbol and the operand type is defined.
    DuplicatePrefixFunc,

    /// Title: DuplicatePostfixFunc
    /// Payload: The symbol of the operator definition and operand value type
    /// An operator function with the same symbol and the operand type is defined.
    DuplicatePostfixFunc,

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
    /// Payload: None
    /// A function requires a return value, but return statement is missing in single or multiple scopes.
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
    /// Payload: The expected type and the actual type of the argument
    /// The type of the argument passed to an builtin function is wrong.
    BuiltinArgTypeMismatch,

    /// Title: ImportNotFound
    /// Payload: The path of the import
    /// An import path is not found.
    ImportNotFound,

    /// Title: CyclicDependency
    /// Payload: The path of the import
    /// An import makes a cyclic dependency.
    CyclicDependency,

    /// Title: ExprEndsWithType
    /// Payload: None
    /// An expression ends with a type.
    ExprEndsWithType,

    /// Title: ExprEndsWithBuiltin
    /// Payload: None
    /// An expression ends with "Builtin" keyword.
    ExprEndsWithBuiltin,

    /// Title: NoMainFunc
    /// Payload: None
    /// The compile target does not have a main function.
    NoMainFunc,

    /// Title: UnmatchedBracket
    /// Payload: None
    /// Bracket [] does not match.
    UnmatchedBracket,

    /// Title: SubscriptOnNonArray
    /// Payload: Type of the non-array value
    /// Subscript access is performed on a non-array value.
    SubscriptOnNonArray,

    /// Title: NonIntegerInSubscript
    /// Payload: Type of the non-integer value
    /// A non-integer value is used to index an array.
    NonIntegerInSubscript,

    /// Title: NonIntegerForCount
    /// Payload: Type of the non-integer count value
    /// A non-integer value is used for array count in array spread literal.
    NonIntegerForCount,

    /// Title: NonConstantForCount
    /// Payload: None
    /// A non-constant value is used for array count in array spread literal.
    NonConstantForCount,

    /// Title: EmptyArrayLiteral
    /// Payload: None
    /// An array with no item is initialized. This is error because KASL only supports fixed-length arrays and empty array is meaningless.
    EmptyArrayLiteral,

    /// Title: ArrayItemTypeMismatch
    /// Payload: The type of the first item and the type of the value which has wrong type
    /// The type of the array item is does not match.
    ArrayItemTypeMismatch,

    /// Title: CompilerBug
    /// Payload: Error message
    CompilerBug,
}
