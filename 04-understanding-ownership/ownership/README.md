# Chapter 4: Understanding ownership

Rust uses the ownership approach to memory management, in contract to GC and manual
management.

The rules are: Each value has only one owner variable. Once the owner goes out of scope,
the value is dropped via the `drop` method. This pattern is analogous to the C++ RAII.

The `String` type instances are allocated on the heap, as opposed to `str`. They're also mutable.

## How copies work for heap-allocated values

This code

```rust
let s1 = String::from("hello");
let s2 = s1;
```

corresponds to the following picture.

![How copies work for heap-allocated values](https://doc.rust-lang.org/book/img/trpl04-04.svg)

The group of data that includes the pointer, the length and capacity are stored on the
stack. These are indeed copied over to `s2`, but not the heap data.

The `s1` variable is _invalidated_, so you can only use `s2` after doing this operation.
Indeed, this “shallow copy” is known as a **move** for this reason. Now, when `s2` goes
out of scope it will free the memory by itself, avoiding the _double free_ error.

A deep copy can be accomplished via the `clone` method.

Note that the move isn't done for certain types like integers, because that's cheap to
do. This is explicitly done when the type implements the **`Copy` trait**.

## Functions

When you pass a variable to a function, its ownership is moved to the function, so the
variable is invalidated in the caller's scope, unless:

1. The function returns the variable, effectively giving back ownership to the caller.
2. We use **references**, like `let len = calculate_length(&s1);`.
