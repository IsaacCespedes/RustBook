//  Unsafe Rust
// you can write an unsafe rust code block with the unsafe keyword
// unsafe rust lets you do the following:
// - Dereference a raw pointer
// - Call an unsafe function or method
// - Access or modify a mutable static variable
// - Implement an unsafe trait
// - Access fields of unions
// unsafe doesn’t turn off the borrow checker
// or disable any other of Rust’s safety check

// Keep unsafe blocks small

unsafe fn dangerous() {}

// Advanced Traits
// associated types can only be implemented for one type
struct Counter;
pub trait Iterator {
    type Item; // Item is the associated type

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
}

// vs generics which can be implemented for multiple types
// and must specify each type

pub trait GenericIterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl GenericIterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(1)
    }
}

impl GenericIterator<String> for Counter {
    fn next(&mut self) -> Option<String> {
        Some("String".to_string())
    }
}

// default generic type
// can be overridden by specifying the type
pub trait GenericIterator2<T = u32> {
    fn next(&mut self) -> Option<T>;
}

// operator overloading
use std::ops::Add;

// implementation has a default trait
//trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// newtype pattern, a thin wrapper around a type
struct Millimeters(u32);
struct Meters(u32);

// default type is overridden
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// fully qualified syntax for disambiguation
// associate functions, unlike methods, don't have a self parameter
// if there are multiple implementations of a trait for an associated function,
// you must use fully qualified syntax
// e.g. <Type as Trait>::function(receiver_if_method, next_arg, ...);
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// let person = Human;
// person.fly(); // *waving arms furiously*

// fully qualified syntax
// Pilot::fly(&person);
// Wizard::fly(&person);

// Requiring one trait to implement another trait
// here we require that the type implements Display

// types that implement the OutlinePrint trait must also implement the Display trait

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string(); // Display trait
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Implementing External Traits on External Types
// orphan rule: you can implement a trait on a type as long as either the trait
// or the type are local to your crate

// here, wrapper does not have the vector methods
// to access all the vector methods, we can implement the Deref trait
// or implement the methods we want manually

//use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// let w = Wrapper(vec![String::from("hello"), String::from("world")]);
// println!("w = {}", w);

// type synonyms
// Kilometers will be treated like i32
type Kilometers = i32;

// useful for creating shorthand of types
type FooType = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;

// the never type (!) indicates that a function never returns
// fn bar() -> ! {
//     // --snip--
// }

// the loop {} expression, and the `continue` and `!panic` statements returns a never type
// ! can be coerced into any type, as seen in match expressions

// Dynamically Sized Types (DSTs)

// e.g. str on its own (not &str), is a DST.
// We can’t know how long the string is until runtime,
// meaning we can’t create a variable of type str,
// nor can we take an argument of type str

// The golden rule of dynamically sized types:
// we must always put values of dynamically sized types behind a pointer of some kind.

// traits are DSTs
// e.g. Box<dyn Trait>

// the Sized trait determines when a type is knoown at compile time
// every generic fuunctio has this bound implicitly
// these are the same:

// fn generic<T>(t: T) {
//     // --snip--
// }

// fn generic<T: Sized>(t: T) {
//     // --snip--
// }

// function pointers
// the fn type is called a function pointer
// Function pointers implement all three of the closure traits
// (Fn, FnMut, and FnOnce)
// so a function pointer can be passed as an argument that expects a closure
// e.g. to iter.map()

// note: enums values that can hold a value can also work as a function pointer
// i.e. be passed into iter.map()

// note: closures need to be wrapped in a Box<> to serve as a function's return type

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Macros: similar to C++
// (metaprogramming, expands during preprocesing, before compiling)
// e.g.  #[derive] too add attributes on structs and enums
// attribute-like macros to define custom attributes
// function-like macros. e.g. println! and vec!

// macro can have multiple arms
// macro patterns can match expresions, types or items
// declarative macros work on patterns
// procedural macros work oon code blocks

// todo: outline notes and code procedural macros
// declarative macros are the most common
// and 'for complex technical reasons that we hope to eliminate in the future'
// procedural macros require a separate crate with a special trait

// e.g. vec!
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// generates this:
// {
//     let mut temp_vec = Vec::new();
//     temp_vec.push(1);
//     temp_vec.push(2);
//     temp_vec.push(3);
//     temp_vec
// }

fn main() {
    // dereferencing raw pointers
    // raw pointers are similar to references
    // can be mutable (*mut T) or immutable (const *T)
    // they can have both immutable and mutable pointers
    // or multiple mutable pointers to the same location
    // Aren’t guaranteed to point to valid memory
    // Are allowed to be null
    // Don’t implement any automatic cleanup

    // they allow the ability to interface with code written in other languages

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // calling an unsafe function or method
    unsafe {
        dangerous();
    }

    // safe abstraction over unsafe code

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // external function
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // mutating static variables
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // operator overloading
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // funuction pointer
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer); // 12
}

// safe abstraction over unsafe code
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // get raw pointer to the slice

    assert!(mid <= len);

    unsafe {
        (
            // this takes a raw pointer and trusts that this pointer is valid
            slice::from_raw_parts_mut(ptr, mid),
            // this trusts that the offset location is also a valid pointer
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// extern functions to call external code
// via Foreign Function Interface (FFI)

// external functions are alwatys unsafe to call
// because Rust can't check themm

// "C" is the ABI (Application Binary Interface) name

extern "C" {
    fn abs(input: i32) -> i32;
}

// allowing other languages to call Rust functions

// mangling is the process of encoding the symbol names
// (e.g. function name) during compilation
// the `unsafe` keyword is not required here
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Accessing or modifying a mutable static variable
// global variables are static in Rust
// static variables are similar to constants
// Unlike constants, they are not allowed to duplicate their data whenever they’re used.
// Static variables can be mutable.
// Accessing and modifying mutable static variables is unsafe
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// unsafe traits
// A trait is unsafe when at least one of its methods has some invariant
// that the compiler can’t verify.

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// Accessing fields of a union
// A union is similar to a struct, but only one of its fields is active at any given time.
// Unions are primarily used to interface with unions in C code.
