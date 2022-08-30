# Logical Expressions
Within Redex, all expressions don't require use of parenthises, and are terminated by newlines or opening brackets by default, though a set of parenthises can be used to encapsulate nested logic within an expression.

## Operators
 - `==` Casting Equality
 - `===` Type Bound Equality
 - `~=` Non-Equality.
 - `~` Not (Will cast to primitive)

 - `if` Denotes a conditional expression block
 - `or` denotes an alternative path for in a aconditional expression block
 - `else` denotes a block of code to execute if no condition is matched.
 - `++` variable incrementing
 - `--` variable decrementing
 - `+=` shorthand sum.
 
 - `*` multiply
 - `-` subtract
 - `/` divide
 - `+` addition
 - `^` exponent

### If Statement
```rs
// A basic condition
if x ~= y {
}

// A compound condition (The inner expression will be evaluated first)
if x == (x + 1) {
}

// A condition chain
if x == y {
} or x == z {
} or a = z {
}

// An else block
if x == y {

} else {
    // :(
}
```

### Expressions
```rs
// The right hand side is evaluated as an expression, with the paraenthesis being evaluated first.
let x = (334 / 22) * 53; // 804.6363~
let y = !1 // false
let z = !0 // true
```