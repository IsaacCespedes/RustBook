// Box<T> for allocating values on the heap
// Rc<T> a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>,
// a type that enforces the borrowing rules at runtime instead of compile time

// Box<T> use cases
// - When you have a type whose size can’t be known at compile time
// and you want to use a value of that type in a context that requires an exact size
// e.g. recursive types like a linked list
// to determine size, e.g. for enums, Rust goes through each of the variants
// to see which variant needs the most space
// - When you have a large amount of data and you want to transfer ownership
// but ensure the data won’t be copied when you do so
// - When you want to own a value and you care only that it’s a type that implements a particular trait
// rather than being of a specific type

// note that without box, the size of List would be infinite
// box provides a layer of indirection
// Box has a size of usize, which is the size of a raw pointer

// Box<T> is a smart pointer because it implements the Deref trait
// which allows Box<T> values to be treated like references
// it also implements the Drop trait, behavior for deallocating memory
// when a Box<T> goes out of scope

// Reference Counting with Rc<T>
// Rc<T> enables multiple ownership
// Rc<T> keeps track of the number of references to a value
// and when there are zero references,
// the value can be cleaned up safely
// use when you want to allocate some data on the heap for multiple parts of a program to read
// and you can’t determine at compile time which part will finish using the data last
// an example use case is with graph data structures
// where multiple edges might point to the same node
// Note that Rc<T> is only for use in single-threaded scenarios

// RefCell<T> allows immutable or mutable borrows checked at runtime.
// it can cause panics if the borrowing rules are violated at runtime
// it is implemented using interior mutability pattern, with unsafe code
// you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable
// RefCell<T>  allows only single ownership over the data it holds
// RefCell<T> is only for use in single-threaded scenarios

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RCList {
    Cons(i32, Rc<RCList>),
    Nil,
}

use crate::List::{Cons, Nil};
use crate::RCList::{Cons as RCCons, Nil as RCNil};
use std::rc::Rc;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let b = Box::new(5); // 5 gets stored on the heap
    println!("b = {}", b);
    // b is deallocated when it goes out of scope
    // and the heap memory is freed

    // box can be treated like a reference
    let x = 5;
    let y = Box::new(x); // x is copied into the box

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // a is moved into b

    // reference counting
    //  Data inside an Rc cannot be mutated without the use of interior mutability
    //  Rc enforces this property by implementing the Deref trait, but not implementing the DerefMut trait.
    let rc_a = Rc::new(RCCons(5, Rc::new(RCCons(10, Rc::new(RCNil)))));
    let rc_b = RCCons(3, Rc::clone(&rc_a)); // Rc::clone doesn't make a deep copy
    let rc_c = RCCons(4, Rc::clone(&rc_a)); // reference count is increased
    println!("reference count of rc_a: {}", Rc::strong_count(&rc_a)); // 3
}
