# Chapter 10: Generic Types, Traits, and Lifetimes

Rust implements generics by **monomorphization**, which means turning generic code into
specific code on compilation, so runtime efficiency is the same.

Traits are defined using the following notation

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Here we're saying that types implementing the `Summary` trait must implement the
`summarize` method. This is done via:

```rust
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // Implementation for NewsArticle struct
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        // Implementation for Tweet struct
    }
}
```

We can't implement external traits on external types (with respect to our crate), but
we can do it if either the trait or the type is local. This restriction is part of the
[**orphan rules**](https://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls/).

For generic functions, we can restrict parameters to those with types that implement
a trait via `impl`. For multiple traits we can use `+`.

```rust
pub fn notify(item: &impl Summary) {}
pub fn better_notify(item: &(impl Summary + Display)) {}
```

This is syntactic sugar for a longer form called **trait bound**:

```rust
pub fn notify<T: Summary>(item: &T) {}
pub fn better_notify<T: Summary + Display>(item: &T) {}
```

This is more flexible for things like requiring the same type for two trait-bound
parameters.

For even less clutter, we can use `where` clauses.

```rust
// Harder to read function signature
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// Cleaner signature with the where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
    {}
```

**Blanket implementations** happen when we conditionally implement a trait for any
type that implements another trait. For example, the standard library implements
`ToString` for any type that implements `Display`.

```rust
impl<T: Display> ToString for T {}
```

## Lifetimes

The **borrow checker** is a part of the compiler that compares scopes to verify the
validity of all borrows.

The notation for lifetime allow us to _reject_ values not adhering to such constraint,
we don't change the lifetimes. For a function signature, write:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}
```

We can also specify lifetimes in structs like so:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

**Lifetime elision rules** are patterns programmed into the compiler that allow us to
not write explicit lifetimes.

The **static lifetime** means that the reference can live for the entire program
duration. String literals have this lifetime, for example.

```rust
let s: &'static str = "I have a static lifetime.";
```

[Previous](/09-error-handling/panic/) | [Next](/11-writing-automated-tests/adder/)
