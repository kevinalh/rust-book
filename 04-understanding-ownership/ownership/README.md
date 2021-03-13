# Chapter 4: Understanding Ownership

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

## Functions and references

When you pass a variable to a function, its ownership is moved to the function, so the
variable is invalidated in the caller's scope, unless:

1. The function returns the variable, effectively giving back ownership to the caller.
2. We use **references**, like `let len = calculate_length(&s1);`.

Passing references as function parameters is called **borrowing**. These references
allow us to work with variables without taking ownership. You can't modify them unless
you pass a **mutable reference** with the `&mut` syntax, but there's a restriction of
one mutable reference allowed per scope. The purpose of this restriction is to avoid
data races.

Important note: The scope of a reference starts when it is introduced and
**ends at the last point where it's used**.

Dangling pointers are also checked for by the compiler.

## Slices

Use the notation

```rust
&s[0..i]
```

String slices have the type `&str`. String literals are slices.

Since they're immutable references, passing a mutable reference to the underlying string
is an error while they're in the scope.

For `i32` arrays, the type of the slice is `&[i32]`.
