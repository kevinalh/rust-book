# Enums and Pattern Matching

## Enums

We can create enums via the `enum` keyword. We can even assign data to each case.

```rust
enum Message {
    Quit,  // No data associated
    Move {x: i32, y: i32},  // Anonymous struct
    Write(String),  // Includes a single String
    ChangeColor(i32, i32, i32),  // Includes three i32 values
}
```

Similarly to structs, we can also use `impl` to define methods on enums.

The `Option<T>` enum allows us to express missing values without having the `null`
concept.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

## Pattern matching

We can use `match` and the compiler will check if we're exhaustive for enums. The `_`
pattern will match any value.

If we only care about one particular case, and maybe a `_`, like

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => println!("not three"),
}
```

Then we can use the `if let` shorthand:

```rust
if let Some(3) = some_u8_value {
    println!("three");
} else {
    println!("not three");
}
```
