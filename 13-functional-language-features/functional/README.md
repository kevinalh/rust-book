# Functional Language Features: Iterators and Closures

## Closures

Closures can be written as

```rust
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

Note we didn't specify the variable types. This is because the compiler can infer them
from usage in the first call. A consequence of this inference is that if you call the
function with a different data type on its input later on, it will fail.

There are three traits that encode the ways a closure can take capture the environment:

- `FnOnce`: Consume the variables from the environment, that is, take ownership of the
  variables and move them into the closure. These can only be called once.
- `FnMut`: Mutably borrows, so can change the environment.
- `Fn`: Borrow values immutably.

By default you can start with `Fn` and the compiler will let you know if you need to
change it.

You can use the `move` keyword to force the ownership of the variables it uses in the
environment. Useful in multi-threading.

## Iterators

The `Iterator` trait requires defining the `next` method. It uses the concept of
**associated type**. They are lazy.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Calling the `next` method changes the internal state of the iterator, so calling it
requires that we make it mutable.

The `iter` method produces immutable references, while `into_iter` takes ownership of
the container and `iter_mut` iterates over mutable references.

A **consuming adaptor** is a method that calls `next`, for example `sum`.

An **iterator adaptor** is a method that produces another iterator, for example `map`.

Iterators are efficient. The **unrolling optimization** is done when the compiler can
determine there's a fixed amount of iterations and instead of doing a loop in assembly,
it creates repetitive code, removing the iteration overhead.

[Previous](/12-building-a-command-line-program/minigrep/) | [Next](/14-cargo-crates/add/)
