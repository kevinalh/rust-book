# Managing Growing Projects with Packages, Crates and Modules

**Packages** can contain multiple **binary crates** and optionally one
**library crate**.

The `src/main.rs` file is, by convention, the crate root of a binary crate with the same
name as the package.

If the package contains `src/main.rs` and `src/lib.rs`, it has both a binary crate and
a library crate, both with the same name as the package. These are the **crate roots**.

To have more **binary crates**, you can put the files inside `src/bin/`

## Modules

We can define them with the `mod` keyword. These can be nested.

To access what's defined inside them, you can use either relative or absolute paths.
The root path is accessed via the `crate` keyword, so you can write:

```rust
crate::front_of_house::hosting::add_to_waitlist();
```

Absolute paths can also be created by using the crate name.

For relative paths, you can use the `super` keyword to access the parent module.

```rust
super::serve_order();
```

To be able to refer to the stuff inside a module, you need to set both the module and
the function (or struct, enum, etc.) to **public** via the `pub` keyword.

One important detail is that while making an enum public makes all of its variants
public, _this is not the case for structs_, where you need to add `pub` behind each
needed field.

A consequence of this is that for structs with private fields, we need to create a
public associated function that constructs an instance of them.

## The `use` keyword

We can bring paths into scope by writing, for example,

```rust
use self::front_of_house::hosting;
```

> Idiomatic usage is to specify only until the module level for functions, and the full
> path for structs, enums and other items (unless there's a clash).

To avoid clashes you can also use the `as` keyword, like

```rust
use std::io::Result as IoResult;
```

When we bring items into scope, they're private by default, unless we use `pub use`.
This is called **re-exporting**.

To import multiple items from the same path, we can use **nested paths**:

```rust
use std::io::{self, Write};
```

which is equivalent to

```rust
use std::io;
use std::io::Write;
```

[Previous](/06-enums-and-pattern-matching/enums/) | [Next](/08-common-collections/collections/)
