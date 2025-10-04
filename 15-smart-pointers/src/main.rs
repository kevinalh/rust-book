use std::rc::Rc;

// Box smart pointers
#[allow(dead_code)]
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::{fmt::Display, ops::Deref};

use crate::List::{Cons, Nil};

// Deref / Drop
struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    // associated type
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping MyBox with data `{}`", self.0)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Using lifetimes
#[allow(dead_code)]
#[derive(Debug)]
enum LifetimeList<'a> {
    Cons(i32, &'a LifetimeList<'a>),
    Nil,
}

#[allow(dead_code)]
#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Moving and going out of scope also triggers a drop
    // In fact, this is pretty much how std::mem::drop works
    {
        y
    };
    let m = MyBox::new(String::from("Rust"));
    // The following two are equivalent thanks to Deref coercion
    hello(&m);
    hello(&(*m)[..]);
    drop(m);

    // Using lifetimes
    let lifetime_list = LifetimeList::Cons(2, &LifetimeList::Cons(3, &LifetimeList::Nil));
    println!("{:?}", lifetime_list);

    // Using Rc
    let rc_list_a = Rc::new(RcList::Cons(
        2,
        Rc::new(RcList::Cons(3, Rc::new(RcList::Nil))),
    ));
    println!("count of rc_list_a: {}", Rc::strong_count(&rc_list_a));
    let rc_list_b = RcList::Cons(3, Rc::clone(&rc_list_a));
    let rc_list_c = RcList::Cons(4, Rc::clone(&rc_list_a));
    println!("count of rc_list_a: {}", Rc::strong_count(&rc_list_a));
    println!("{:?}; {:?}", rc_list_b, rc_list_c);
}
