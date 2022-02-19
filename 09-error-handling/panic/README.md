# Error Handling

By default, Rust does **stack unwinding** on panic, but you can choose to **abort**
instead, letting the OS reclaim the memory instead. Setting this option results in
smaller binaries.

You can execute a panic by calling the `panic!` macro.

The `Result` type is defined as

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `?` operator can be used to return the error value without having to write a `match`
statement, effectively **propagating** the error. This will also automatically convert
the type to the one specified as the function return type, via the `from` function.

The `Box<dyn Error>` type is a **trait object**, which basically covers any kind of
error. Hence, if you want to use the `?` operator inside the `main` function, you can
make `main` return `Result<(), Box<dyn Error>>`.

You can put your validation code in the `new` implementation of a `struct`.

[Previous](/08-common-collections/collections/) | [Next](/10-generic-types-traits-and-lifetimes/chapter10/)
