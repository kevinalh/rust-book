# Writing Automated Tests

By using `cargo new ___ --lib`, you get a test skeleton.

The `#[test]` attribute above a function marks it as a test. We can then run the suite
with:

```rust
cargo test
```

Inside these functions we can use the `assert!`, `assert_eq!` and `assert_ne!` macros.
Adding the `#[should_panic]` attribute to the function checks whether the test panics.
This attribute can take the `expected` parameter to check whether the panic message
contains a specific substring.

By default, the tests are run in parallel over threads.
The `--show-output` flag lets us see the standard output of successful tests, which is
otherwise hidden.

The `#[ignore]` attribute can be used to annotate expensive tests. The `--ignored`
flag runs just these.

## Conventions

For _unit tests_, the convention is to put the test in each file next to the code that
is being tested. These should be included in a `tests` module properly annotated with
`#[cfg(test)]`.

For _integration tests_, we create a `tests` directory at the top level of the project.
This directory is treated specially by Cargo. It will compile each of the files here
as an individual crate, but will only compile them when running `cargo test`.
Only test the public-facing API here (those with the `pub` keyword).

To have common functions to be used across tests in different files, we can put them in
a folder, say `tests/common`, and add the `tests/common/mod.rs` file to tell Rust we
don't have tests in there.

The `main.rs` file doesn't need to be tested if the important functionality is in the
library crate.
