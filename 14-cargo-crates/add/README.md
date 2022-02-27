# More about Cargo and Crates.io

The two main profiles are `dev` (when using `cargo build`, includes `debuginfo`) and `release` (with
`cargo build --release`). You can customize these profiles as follows

```toml
[profile.dev]
opt-level = 1
```

The `opt-level` setting is for the number of optimizations, from 0 (used by default in `dev`) to 3
(for `release`).

## Documentation

A **documentation comment** uses three slashes (`///`), and supports Markdown. This will generate
HTML documentation into `target/doc` running `cargo doc --open`, which will use the `rustdoc` tool.

For a function, it's important to include an _Examples_ section, and it's common to also have
_Panics_, _Errors_ (if `Result` is returned) and _Safety_ (in case there's usage of `unsafe`).

Adding examples in the documentation will also be interpreted as tests for `cargo test`, in a way
similar to Python's `doctest`.

Using `//!` is used for adding documentation to the container, like the crate itself at the
beginning of the file.

## Making crates easier to use

You'll want to use `pub use` to bring modules from deeper in the hierarchy to upper levels, so that
users of the crate can find them more easily.

```rust
//! # Art
//! A library for modeling artistic concepts

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
}

pub mod utils {
}
```

You can publish a crate with `cargo publish` and make a version not available to future installs
(because the version might be broken) with `cargo yank --vers {semver}`. It's not possible to delete
code from crates.io, so be careful.

## Workspaces

A **workspace** is a set of packages that share the same `Cargo.lock` and output directory. You need
to publish each one individually to crates.io.

Using `cargo install` allows you to download binaries into `$HOME/.cargo/bin` (by default). This is
useful for quickly using tools.

[Previous](/13-functional-language-features/functional/)
