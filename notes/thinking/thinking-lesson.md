# Rustlings: variables4

## Context

Original exercise (the line `x = 5;` must not be changed):

```rust
// TODO: Fix the compiler error.
fn main() {
    let x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
```

My fix: `let mut x = 3;`

## Question 1

Before looking at the fix: what do you think the compiler said about `x = 5;` in the original, and why did it refuse?

Then, more broadly: Rust could have allowed reassignment by default, like most languages. What do you think is the benefit of making the programmer opt in with `mut`?

### Your answer

(write here)
