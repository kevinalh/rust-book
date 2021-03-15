# Common Collections

## Vectors

Vectors allow you to store a variable amount of items of the same type on the heap, next
to each other.

```rust
let v: Vec<i32> = Vec::new();  // Type is required because it's empty
let v = vec![1, 2, 3];  // Rust can infer the type with the vec! macro
```

Vectors have the usual operations `push` and `get` (which returns an `Option`).

Other than `get`, we can use indexing notation:

```rust
let v = vec![1, 2, 3, 4, 5];
let tenth_element = &v[100];
```

But this will panic because we tried to access an out-of-bounds index.

To modify the elements on an iteration, we can use a `for` loop on a mutable reference:

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

If we don't need modifying, we can just use `&v` instead.

## UTF-8 encoded text

Both the `String` and the string slice `&str` types are UTF-8 encoded. In libraries, the
owned and borrow variants of strings tend to end in `String` and `Str`, respectively.

Internally, **a `String` is a wrapper over a `Vec<u8>`**.

Types that implement the `Display` trait have the `to_string` method.

The `push_str` method appends a string slice to the end, while just `push` can add a
single character.

The compiler can _coerce_ a `&String` into a `&str` when passing to a function. This is
called **deref coercion**.

For concatenating strings, we can add a slice, but note that this doesn't copy and the
added-to string is invalidated. This is because of the signature:

```rust
fn add(self, s: &str) -> String {}
```

Alternatively, if we don't want ownership to be taken we can use the `format!` macro:

```rust
let s = format!("{}-{}-{}", s1, s2, s3);
```

There are three ways to think of an “element” of a String.

1. **Bytes**: Using the `bytes()` method to iterate.
2. **Scalar values**: Using the `char()` method to iterate.
3. **Grapheme clusters**: Not available to iterate on the standard library, complex.

You need to be careful with string slices because Rust will panic if the slice doesn't
end on a char boundary.

## Hash Maps

Available as `std::collections::HashMap`. We can create one via `::new()`, and add
elements via the `insert` method.

Alternatively, we can use the `collect` method with `zip`, for example.

To update a value based on an old one, the pattern is to use `or_insert`:

```rust
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```
