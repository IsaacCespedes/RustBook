// module tree
// crate is implicit, formed from main.rs or lib.rs
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
// If you want to make an item like a function or struct private, you put it in a module.

// you only need to load a file using a mod declaration once in your module tree
mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    // structs and fields are private by default
    // same for tuple structs like pub struct Point(pub i32, pub i32);
    pub struct Breakfast {
        // indivudal fields must be made public
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // we couldn’t create an instance of Breakfast
        // because the seasonal_fruit field of Breakfast is private
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // enums are also private by default
    // but all of its variants are automatically public
    // when the enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // super is like going up a directory with `../`
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// use keyword is like a symbolic link
// similar to use keyword in c++
// this shortcut is limited to the scope
// an idiomatic way to bring a function into scope is to specify the parent
use crate::front_of_house::hosting;

// example of aliasing with `as` keyword,
// another idiomatic way to bring a function into scope
// use crate::front_of_house::hosting as hosting_alias;

// `pub use` allows for re-exporting

// nested paths
// use std::{cmp::Ordering, io};
// and
// use std::io::{self, Write};
// vs
// use std::cmp::Ordering;
// use std::io;
// and
// use std::io;
// use std::io::Write;

// glob operator
// brings all public items into the current scope
//use std::collections::*;

// choosing relavtive or absolute paths
// is based on whether the function will stay in the same module
pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
