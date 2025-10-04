# Chapter 15: Smart Pointers

Smart Pointers are implemented using structs that implement the `Deref` and `Drop`
traits.

## `Box<T>`

The `Box<T>` smart pointer allows allocating values on the heap, what remains on the
stack is the pointer to the heap data. **The borrowing rules are enforced at
compile-time**.

Used when:

- You don't know the size of a type at compile-time and need to use a value of that type
  in a context that required an exact size.
- There's a lot of data, want to transfer ownership but want to ensure data won't be
  copied.
- You want to own a value but only care that its type implements a particular trait.

An example of the first case is _recursive types_, like cons lists. If you have

```rust
enum List {
  Cons(i32, List),
  Nil,
}
```

you will get an error ("recursive type `List` has infinite size"). So you need a `Box`
containing the List, so that the Rust compiler can know how much space this struct will
take (`Box<List>` only takes the size of the pointer).

## `Deref` and `Drop` traits

If you implement `std::ops::Deref` for a type, the compiler will know to interpret
`*y` as `*(y.deref())` under the hood, where in the latter the `*` is just a plain
dereference. This replacement is done for each `*` in our code.

For example

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // associated type
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

Here the dereferences for an `m: MyBox<String>` would be:

- `m.deref()`: `&String`
- `&(*m)`: `&String`
- `*m`: `String`
- `m.deref().deref()`: `&str`
- `&(*m)[..]`: `&str`
- `&m`: `&MyBox<String>`

Implicit **Deref coercions** can convert a reference to a type implementing the `Deref`
trait into a reference to another type, matching a function definition's requirements.
See the attached code for an example:

```rust
fn hello(name: &str) {todo!()}

let m = MyBox::new(String::from("Rust"));
// The following two are equivalent thanks to Deref coercion
hello(&m); // &m is of type &MyBox<String>
hello(&(*m)[..]); // it gets coerced into &str
```

## `Rc<T>`

`Box` owns the value that they hold, so you can't create two boxes for the same value.
In these cases, you can use `Rc`.

Cloning is very cheap and can be done with the plain `.clone()` but it's a convention to
use the more explicit `Rc::clone` to easily differentiate which clones are the expensive
ones when optimizing. Cloning is the operation that increases the reference count, which
you can get through `Rc::strong_count`.

Dropping is called only when the reference count is zero. Calling drop explicitly on the
Rc smart pointers will just decrease the reference count.

## `RefCell<T>` and `RefMut<T>`

`RefCell<T>` **enforces the borrowing rules at run-time**, so your program panics if
they're broken. Like `Rc`, it's only for single-threaded scenarios.

The **interior mutability** pattern is mutating the value inside an immutable value.
`RefCell<T>` allows this. `Mutex` is the thread-safe version of `RefCell`.

A use case is Mock Objects.

[Previous](/14-cargo-crates/add/)
