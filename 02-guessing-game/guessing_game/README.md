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

## Dependency management

You can go to [Crates.io](https://crates.io/) and search for the Crate you want. They
include what to copy to your `Cargo.toml` file.

As of this writing, there's no built-in command line way of doing this operation
without editing the file, but there's third-party Cargo subcommands that do this.

There's [petitions to add this functionality to Cargo proper](https://github.com/rust-lang/cargo/issues/5586)
though, so it's possible they add it at some point.

[Previous](/01-getting-started/hello_cargo/) | [Next](/03-programming-concepts/variables/)
