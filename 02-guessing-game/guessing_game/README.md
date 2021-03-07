# Chapter 2: Programming a Guessing Game

The [Rust Prelude](https://doc.rust-lang.org/stable/std/prelude/index.html) at
`std::prelude` is the list of things automatically imported by Rust.

For other things, we have to use the syntax

```rust
use std::io;
```

The `String` type has the `new` **associated function** that can be used via the `::`
syntax

```rust
let mut guess = String::new();
```

This is similar to what other languages call _static methods_. `::new()` returns an
instance of the type, in this case `String`.

## The `Result` type

Some methods return a value of type `Result`. In our case, `read_line` returns an
`io::Result` (which is itself a specialization of the generic `Result`).
These are **enumerations** whose **variants** are `Ok` or `Err`.

The [`expect`](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.expect)
method can be called on these `Result` values to either return the usable value if it
resulted in `Ok`, or crash and take some action if it's an `Err`.

One very nice feature is that if you forget to handle the `Err` case, you'll get a
warning.
