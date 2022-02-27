# Chapter 12: Building a Command Line Program

As a best practice, the `main` function should only be in charge of code that can be
easily verified by reading it. All the functionality should be done in different
functions: A configuration-parsing function and a `run` function that receives the
configuration is one way to keep code clean.

These functions should return a `Result` type so that the `main` function can handle the
error cases.

You can use the `std::env` module for dealing with environment variables.

The `eprintln!` macro will princt to stderr instead of stdout.

[Previous](/11-writing-automated-tests/adder/) | [Next](/13-functional-language-features/functional/)
