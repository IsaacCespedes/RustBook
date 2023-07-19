fn main() {
    let a = Box::new([0; 1_000_000]); // boxes are smart pointers

    // If a variable owns a box
    // when Rust deallocates the variable's frame
    // then Rust deallocates the box's heap memory.
    let b = a; // a is moved to b

    // println!("{:?}", a); // error: use of moved value: `a`
    println!("{}", b[0]);

    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");

    // clone
    let full_name_clone = full.clone();
    println!("{full_name_clone}");

    // functions can return ownership
    let string_from_function = send_out_string();
    println!("{string_from_function}");

    // References Are Non-Owning Pointers
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} brave new {}", m1, m2);
    println!("{}", s);

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value,
             //     so x points to the value 2
    println!("{} {}", a, x); // prints "1 2"

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let b: i32 = **r1; // two dereferences get us to the heap value
    *x += 1;
    println!("{} {}", b, x); // prints "2 3"

    let r2: &i32 = &*x; // r2 points to the heap value directly
    let c: i32 = *r2; // so only one dereference is needed to read it
    *x += 1;
    println!("{} {}", c, x); // prints "3 4"

    // Pointer Safety Principle: data should never be aliased and mutated at the same time.

    // permissions (read, write, own) are defined on paths and not just variables
    // e.g. a[0], *a, a.0, a.field, and combinations thereof

    // Creating a reference to data ("borrowing" it)
    // causes that data to be temporarily read-only until the reference is no longer used.

    // mutable vs immmutable references
    // you can have 1 mutable reference or many immutable references
    let mut vec: Vec<i32> = vec![1, 2, 3];
    // let num: &i32 = &vec[2];
    // *num += 1; // error: cannot assign to `*num` which is behind a `&` reference
    // vec.push(4); // error: cannot borrow `vec` as mutable because it is also borrowed as immutable
    println!("vec[1] {}", vec[1]); // vec is still readable
    let mut_num: &mut i32 = &mut vec[2];
    // vec.push(4); // cannot borrow `vec` as mutable more than once at a time
    // println!("vec[1] {}", vec[1]); // vec is not still readable
    *mut_num += 1;

    // Permissions Are Returned At The End of a Reference's Lifetime

    println!("Third element is {}", *mut_num);

    // Slices
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("first word is {}", word);

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    assert_eq!(slice, &[2, 3]);
}

// Each value must be owned by exactly one variable.
// Rust deallocates the value once its owner goes out of scope.
// Ownership can be transferred by moves, which happen on assignments and function calls.
fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn greet(g1: &String, g2: &String) {
    // note the ampersands
    println!("{} {}!", g1, g2);
}

// &String will be coerced to &str slice
// slices are not heap allocated
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn send_out_string() -> String {
    let s = String::from("hello string from function");
    s
}
