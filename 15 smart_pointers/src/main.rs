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
// (Mutex<T> is the thread-safe version of RefCell<T>)
// RefCell<t> tracks the count of immutable and mutable references at runtime
// and follows the same borrowing rules as the compiler
// incurs a small runtime performance penalty as a result of keeping track of the borrows at runtime

enum List {
    // to modify the value inside the RefCell<T>
    // Cons(Rc<RefCell<i32>>, Rc<List>),
    Cons(i32, Box<List>),
    Nil,
}

enum RCList {
    Cons(i32, Rc<RCList>),
    Nil,
}

#[derive(Debug)]
enum CycleList {
    CycleCons(i32, RefCell<Rc<CycleList>>),
    CycleNil,
}

impl CycleList {
    fn tail(&self) -> Option<&RefCell<Rc<CycleList>>> {
        match self {
            CycleCons(_, item) => Some(item),
            CycleNil => None,
        }
    }
}

#[derive(Debug)]
struct TreeNode {
    value: i32,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

use crate::CycleList::{CycleCons, CycleNil};
use crate::List::{Cons, Nil};
use crate::RCList::{Cons as RCCons, Nil as RCNil};
use std::cell::RefCell;

use std::rc::{Rc, Weak};

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

    // with RefCell
    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // let b = RCCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = RCCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // // borrow_mut is called on value,
    // // which returns a RefMut<T> smart pointer
    // *value.borrow_mut() += 10;

    // println!("a after = {:?}", a); // Cons(RefCell { value: 15 }, Nil)
    // println!("b after = {:?}", b); // Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    // println!("c after = {:?}", c); // Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))

    let a = Rc::new(CycleCons(5, RefCell::new(Rc::new(CycleNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(CycleCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b); // link points to b, creating a cycle
    // }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // Weak references don’t express an ownership relationship,
    // and their count doesn’t affect when an Rc<T> instance is cleaned up

    // TreeNode in leaf now has two owners: leaf and branch
    let leaf = Rc::new(TreeNode {
        value: 3,
        parent: RefCell::new(Weak::new()), // parent is a weak reference
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(TreeNode {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // downgrade creates a weak reference

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let r1 = Rc::new(0);
    let r4 = {
        let r2 = Rc::clone(&r1);
        Rc::downgrade(&r2)
    };
    let r5 = Rc::clone(&r1);
    let r6 = r4.upgrade();
    println!(
        "strong count {} weak count {}",
        Rc::strong_count(&r1),
        Rc::weak_count(&r1)
    ); // 3 1
       // The three strong refs are r1, r5, and r6.
       // The one weak ref is r4. r2 is dropped at the end of its scope.
}
