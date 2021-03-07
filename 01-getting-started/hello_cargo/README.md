# Chapter 1: Getting Started

## Cargo

You can create a new project via

```
cargo new hello_cargo
```

This created a new Git project with its respective `.gitignore` if we're not in one yet.

We can then add code in the `src/` folder. Afterwards we can compile the code in one go
using

```
cargo run
```

To check if the code compiles without creating the executable we can use

```
cargo check
```

This is much faster so it's useful during development. Finally, we can compile the code
with optimizations by running

```
cargo build --release
```

The executables are inside the `target/release/` folder.

## Language

The presence of `!` in `println!` signals that it's a Rust **macro** instead of a
normal function.

## Toolchain

Formatting can be done via the [`rustfmt`](https://wiki.archlinux.org/index.php/rust#Rustfmt)
tool. This can be configured as to autoformat on VSCode.
