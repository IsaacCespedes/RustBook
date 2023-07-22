fn main() {
    let a = Box::new([0; 1_000_000]); // boxes are smart pointers

    // If a variable owns a box
    // when Rust deallocates the variable's frame
    // then Rust deallocates the box's heap memory.
    let b = a; // a is moved to b

    // println!("{:?}", a); // error: use of moved value: `a`
    println!("{}", b[0]);

    let first = String::from("Ferris");
    // string was moved, can not use first anymore
    // string does not implement Copy trait
    // this would work with integers

    // Moved heap data principle:
    // if a variable x moves ownership of heap data to another variable y,
    // then x cannot be used after the move.
    let full = add_suffix(first);
    println!("{full}");

    // clone can produce a copy of heap data
    // so the original variable can still be used
    let full_name_clone = full.clone();
    println!("{full_name_clone}");

    // functions can return ownership
    let string_from_function = send_out_string();
    println!("{string_from_function}");

    // References Are Non-Owning Pointers
    // they change permissionss on paths
    // until the reference is no longer used
    // then the permissions are returned
    // references must always point to valid data
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} brave new {}", m1, m2);
    println!("{}", s);

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads(dereferences) the heap value, so a = 1
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
    // you can have 1 mutable reference or many immutable (shared) references
    let mut vec: Vec<i32> = vec![1, 2, 3];
    // let num: &i32 = &vec[2];
    // *num += 1; // error: cannot assign to `*num` which is behind a `&` reference
    // vec.push(4); // error: cannot borrow `vec` as mutable because it is also borrowed as immutable (num)
    println!("vec[1] {}", vec[1]); // vec is still readable
    let mut_num: &mut i32 = &mut vec[2];
    // vec.push(4); // cannot borrow `vec` as mutable more than once at a time
    // println!("vec[1] {}", vec[1]); // vec is not still readable
    *mut_num += 1;

    // Permissions Are Returned At The End of a Reference's Lifetime

    println!("Third element is {}", *mut_num);

    let mut name = (String::from("Ferris"), String::from("Rustacean"));

    // this borrows name and name.0
    // but not name.1
    // though the compiler does not know that name.1 is not borrowed
    // when a reference to name is passed to a function
    let first = &mut name.0;

    let mut arr = [0, 1, 2, 3];
    // the compiler will not allow borrowing for any other index after this
    // `cannot borrow `a[_]` as immutable
    // because it is also borrowed as mutable`
    // split_first_mut function is a workaround
    let arr_x = &mut arr[0];

    // Slices are a references with length
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("first word is {}", word);

    // string literals are slices
    // string literals are immutable
    let s = "hello world";

    // slices can be used with arrays
    // they are also a reference and length
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    assert_eq!(slice, &[2, 3]);

    let words = vec!["hello".to_string()];
    // ownership of `words` is moved to `d`
    let d = new_document(words);

    // .to_vec() converts &[String] to Vec<String> by cloning each string
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);

    let word_to_add = "world".to_string();
    // d2 > explicitly < requires a mutable reference in order to update, word_to_add is moved
    // this is unlike scripting languages, where data can be modified through multiple variables
    add_word(&mut d2, word_to_add);

    // The modification to `d2` does not affect `d`
    // get_words returns a reference
    assert!(!get_words(&d).contains(&"world".into()));
}

// Each value must be owned by exactly one variable.
// Rust deallocates the value once its owner goes out of scope.
// Ownership permits moves, which happen on assignments and function calls.
fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

// note the ampersands
fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}

// &String or String will be coerced to &str slice
// therefore &str is preferred over String for function arguments
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

type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}
