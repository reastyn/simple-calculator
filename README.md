# Simple calculator

Implementation of simple calculator in Rust using simple grammar

A -> E END
E -> T + T | T - T
T -> F * F | F / F
F -> NUMBER

## Usage

`cargo run <<< "14 / 3"`
`cargo run <<< "3 * 20 + 5 * 2 - 10 / 2"`