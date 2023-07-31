use generics_traits_lifetimes::aggregator::{Summary, Tweet};
use std::fmt::Display;

// Generics

// a generic type T has no capabilities unless specified:
// it cannot be printed, cloned, or mutated
// (although it can be dropped).
fn largest<T: Ord>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// types must be the same for x and y here
struct Point<T> {
    x: T,
    y: T,
}

// generic methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// methods for specific types
// you can't implement methods for generic and specific types with the same name
// the compiler can not pick which one to use when the specific type is used
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// trait bound conditions on impl
// only for types that implement the Display trait
impl<T: Display> Point<T> {
    fn other_x(&self) -> &T {
        &self.x
    }
}

// traits are similar to interfaces in other languages
pub trait myTrait {
    fn myTraitMethod(&self) -> String;
}

// implement the trait for a specific type
impl<T: Display> myTrait for T {
    fn myTraitMethod(&self) -> String {
        format!("Display String")
    }
}

// generics with different scopes
struct MixPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixPoint<T, U> {
    fn mixup<R, S>(self, other: MixPoint<R, S>) -> MixPoint<T, S> {
        MixPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// this is how to specify multiple types
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// generics in enums
// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// trait as parameter
// syntactic sugar for this trait bound
//  pub fn notify<T: Summary>(item: &T)
// it is also possible to specify multiple trait bounds on a single parameter
//  pub fn notify(item: &(impl Summary + Display)), or
// pub fn notify<T: Summary + Display>(item: &T) or
// pub fn notify<T>(item: &T)
// where T: Summary + Display
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// traits as return types
// can only return a single type
// (e.g. only Tweet or only NewsArticle,
// but can not return either in the function via if stmt)
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// lifetimes annotations/specifiers inform the compiler
// about the relationship of lifetimes between reference parameters and return values
// the main aim of lifetimes is to prevent dangling references

// e.g.
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// the lifetime of a reference returned
// is the same as the smaller of the lifetimes of the values referred to by the arguments
// the return value lifetime must match one of the input lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lifetimes in structs
// references in structs must have a lifetime specifier
// the lifetime of the reference in the struct must be the same as the lifetime of the struct
// the compiler will reject the struct if the lifetimes are not the same
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime elision rules
// 1. each parameter that is a reference gets its own lifetime parameter
// 2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. if there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters

// static lifetime
// the lifetime of the entire program
// all string literals have the static lifetime
// let s: &'static str = "I have a static lifetime.";

// combining generics, traits, and lifetimes
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // note: the trait Summary must be in scope to use the summarize method
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.default_summary());

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
