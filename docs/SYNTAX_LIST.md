## Comments
```
// <comment>
```

## Input Decl
```
input <name>: <type?> = <default?>
```
## Output Decl
```
output <name>: <type>
```

## Var Decl
```
var <name>: <type?> = <default>
```

## State
```
state {
    <name>: <type?> = <default>
}
```

## Func
```
func <name>((<label?> <name>: <type?> = <default?>)*) -> <type?> {
    <body>
}
```

## Return
```
return <value>
```

## If
```
if <condition> {
    <body>
}
```

## If-else
```
if <condition> {
    <body>
} else {
    <body>
}
```

## Struct
```
struct <name>: <type?>, <protocol>* {
    var <name>: <type?> = <default>

    init((<label?> <name>: <type?> = <default?>)*) {
        self.<name> = <default>
    }

    func <name>((<label?> <name>: <type?> = <default?>)*) -> <type?> {
        <body>
    }
}
```

## Protocol
```
protocol <name>: <type?>, <protocol>* {
    func <name>((<label?> <name>: <type?> = <default?>)*) -> <type?>
}
```

## Protocol Inheritance
```
struct <name>: <type?>, <protocol>* {
    // ...

    <protocol> func <name>((<label?> <name>: <type?> = <default?>)*) -> <type?> {
        <body>
    }
}
```

## Literal Binding
```
intliteral init((<label?> <name>: <type?> = <default?>)*) {
    self.<name> = <default>
}

floatliteral init((<label?> <name>: <type?> = <default?>)*) {
    self.<name> = <default>
}

boolliteral init((<label?> <name>: <type?> = <default?>)*) {
    self.<name> = <default>
}
```

## Raw Reference
Use `raw` keyword to access the inheritance source
```
raw
```

## Operator Decl
```
infix <symbol>(rhs: <type>) -> <type> {
    associativity: <associativity>
    priority: <priority>
}: {
    <body>
}

prefix <symbol>() -> <type> {
    <body>
}

postfix <symbol>() -> <type> {
    <body>
}
```
