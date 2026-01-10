# 2. Language Basics

This section covers the fundamental building blocks of the KASL language, including comments, variables, constants, and the basic data types you'll use in every program.

---

## Comments

Comments are parts of the code that are ignored by the compiler. They are useful for adding explanations or notes within your code for yourself and other developers.

KASL uses C-style single-line comments, which start with `//`.

```/dev/null/example.kash
// This is a single-line comment. The compiler will ignore this entire line.

let a = 10 // You can also put comments at the end of a line.
```

---

## Variables and Constants

In KASL, you store values in named containers. There are two kinds: constants and variables.

### Constants (`let`)

Use `let` to declare a **constant**, which is a value that cannot be changed after it's assigned.

```/dev/null/example.kash
let pi = 3.14159
let sampleRate = 44100

// This would cause a compile-time error because `pi` is a constant.
// pi = 3.0
```

It's good practice to use constants for values that you know won't change, as it makes your code safer and easier to understand.

### Variables (`var`)

Use `var` to declare a **variable**, which is a value that can be changed later.

```/dev/null/example.kash
var volume = 0.5
volume = 0.8 // This is allowed.

var counter = 0
counter = counter + 1 // You can modify variables.
```

---

## Basic Data Types

KASL comes with a set of common, built-in data types. When you declare a variable or constant, the compiler can often infer the type from the value you assign.

### Integers (`Int`)

Integers are whole numbers, like `-1`, `0`, and `42`.

```/dev/null/example.kash
let myInteger = 100
var currentStep = -1
```

### Floating-Point Numbers (`Float`)

Floating-point numbers are numbers with a fractional component, like `3.14`, `-0.5`, or `10.0`. These are essential for audio processing.

```/dev/null/example.kash
let frequency = 440.0
var amplitude = 0.99
```

### Booleans (`Bool`)

Booleans represent truth values. They can only be `true` or `false`.

```/dev/null/example.kash
let isEnabled = true
var hasFinished = false
```
