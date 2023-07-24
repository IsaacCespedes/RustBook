// Closures

// closures are anonymous functions that can capture their environment
// like arrow functions in JS

// they can implement the Fn, FnMut, and FnOnce traits

// they can trigger move semantics and borrows like fn functions
// let mut list = vec![1, 2, 3];
// println!("Before defining closure: {:?}", list);
// let mut borrows_mutably = || list.push(7);
// //can not use list here because it is borrowed by the closure
// borrows_mutably();
// println!("After calling closure: {:?}", list);

// you can force a move with the move keyword
// let mut list = vec![1, 2, 3];
// println!("Before defining closure: {:?}", list);
// let mut borrows_mutably = move || list.push(7);
// borrows_mutably();

// Closures don’t usually require you to annotate the types of the parameters
// or the return value like fn functions do.
// closures can not be called with multiple types, for example:
// let example_closure = |x| x;
// let s = example_closure(String::from("hello"));
// let n = example_closure(5);

// closures can also capture lifetimes, e.g.:
// fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
//     move || s_ref.to_string()
// }
// // or
// fn make_a_cloner(s_ref: &str) -> impl Fn() -> String + '_ {
//     move || s_ref.to_string()
// }

// Iterators
// they implement the Iterator trait, which has a next method
// and an Item associated type

// iter() returns immutable references
// so next() returns immutable references

// into_iter() returns owned values
// iter_mut() returns mutable references

// methods like sum() and collect() consume the iterator
// so you can not use it after that
// v1_iter.sum();
// methods like map() and filter() (iterator adapters) do not consume the iterator
// v1.iter().map(|x| x + 1).collect();
// and you you must call a consumer for the iterator to activate

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // closures hold the parameters in the pipes ||
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
