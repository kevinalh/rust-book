# Chapter 3: Common Programming Concepts

**Constants** are defined via the `const` keyword. They always require a type annotation
and can never be mutable. They also can only be set via a _constant expression_ (that
is, no function call).

**Shadowing** is when you have a variable declared and assigned, and then you call `let`
again with the same name. You can even change the variable type this way.

**Arrays** in Rust have fixed length, and all elements are of the same type. The type
signatures look like:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

If you want an array of the same element multiple times, you can use

```rust
let a = [3; 5];
// Equivalent to...
let a = [3, 3, 3, 3, 3];
```

Trying to access an array element with an out-of-bounds index causes the runtime to
panic.

**Tuples** can have elements of different types. Each element can be accessed via dot
notation and the number corresponding to the element position.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
```

**Statements** are used for their side effects and don't return a value, while
**expressions** do have a returning value. **Adding a semicolon to the end of an**
**expression turns it into a statement**.

The block used to create new scopes is an expression that evaluates to the last
line. Note that it shouldn't include a semicolon.

```rust
let y = {
    let x = 3;
    x + 1
} // y is assigned the value 4
```

Similarly, in functions you don't add the semicolon to the last expression and it's
returned implicitly (although you can also use the `return` keyword).

If no value is returned from a function, the implicit return is the empty tuple `()`.

**For loops** can be used to safely iterate through an array via their `iter` method:

```rust
let a = [10, 20, 30, 40, 50];
for element in a.iter() {}
```

Range iterations can be done via the `Range` type:

```rust
for number in (1..4).rev() {}
```
