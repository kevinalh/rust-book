# Using Structs to Structure Related Data

We use the keyword `struct` to define one, like this:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

Storing references in structs require the user of **lifetimes**.

Another feature is that of **tuple structs**, where each field doesn't have a name but
only position. We do define the types beforehand though.

```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

Each tuple struct has its own type.

## Annotations

We can use the `derive` annotation to add behavior to types. For example, to add the
`Debug` trait we can write:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

## Methods

We can add methods that operate on structs. These take a `self` parameter like in other
languages. Usually, the `self` parameter is taken as an immutable reference to avoid
taking ownership of it, but the other cases are possible too.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        // implementation
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        // implementation
    }
}
```

Thanks to “**automatic referencing and dereferencing**”, we can write

```rust
rectangle.area();
```

instead of

```rust
(&rectangle).area()
```

An **associated function** is like a method inside a `impl` block, but it doesn't take
the `self` parameter. These are then called with the `::` syntax.
