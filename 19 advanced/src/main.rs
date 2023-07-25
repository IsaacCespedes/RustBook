// yoou can write an unsafe rust code block with the unsafe keyword
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
