# Getting Started

You can create a new project via

```
cargo new hello_cargo
```

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
