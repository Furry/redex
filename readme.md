<h1 align="center">Redex</h1>
<p align="center"> A proof of concept scripting langauge built from the ground up.<p>

## Syntax
Based on a hybrid syntax of Lua & Rust, it follows a similar syntax to both languages, but with a few key differences, the most notable being the following;
- Semi-colons are optional, but recommended.
- Negation for booleans is done with the `~` operator.
- String concatenation is done with the `+` operator.
- Excepts are handled with an additional return value, rather than a seperate construct.
- Conditional chaining is done with the `or` keyword.
- All integers are floating point, and are unsized.

## Lexing
The custom lexing engine is a very light implementation, providing simple tokenization for strings and literals with a few special cases for operators and keywords with whitespace taken into account for perserving the structure of the code in the AST.

## Parsing
The parser is a recursive descent parser, utilizing a pair of dequeues to store the tokens and the AST nodes. The parser is very simple, and is only used to parse the code into a tree structure, with the AST being built from the bottom up. While still very much a work in progress, it supports rough lexing of mathmatical expressions.