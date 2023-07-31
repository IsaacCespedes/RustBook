#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // enums can store data inside them
                      // and each variant can store different types and amounts of data
}

// enums can also have impl blocks and methods
// inside an enum method, self will be the value of the enum variant the method is being called on

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// these two functions are the same
// Option is an enum of Some<T> and None (which has no data)
// Rust does not offer the null value,
// but it does have an enum that can encode the concept of a value being present or absent
fn decr_twice_v1(n: u32) -> Option<u32> {
    match n {
        0 => None,
        1 => None,
        n2 => Some(n2 - 2), // catch all, n2 is the value of n
    }
}
fn decr_twice_v2(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else if n == 1 {
        None
    } else {
        Some(n - 2)
    }
}

fn main() {
    let penny = Coin::Penny;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Alaska);

    println!("Penny: {}", value_in_cents(penny));
    println!("Dime: {}", value_in_cents(dime));
    println!("Quarter: {}", value_in_cents(quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Five: {:?}", five);
    println!("Six: {:?}", six);
    println!("None: {:?}", none);

    let dice_roll = 7;
    match dice_roll {
        1 => println!("Rolled a 1!"),
        2 => println!("Rolled a 2!"),
        3 => println!("Rolled a 3!"),
        4 => println!("Rolled a 4!"),
        5 => println!("Rolled a 5!"),
        6 => println!("Rolled a 6!"),
        _ => println!("Rolled something else!"), // catch all, does not use dice_roll
    }

    match dice_roll {
        1 | 2 | 3 => println!("Rolled a 1, 2, or 3!"),
        4 | 5 | 6 => println!("Rolled a 4, 5, or 6!"),
        other => println!("Rolled {other}!"), // catch all, uses dice_roll
    }

    let opt: Option<String> = Some(String::from("opt string"));

    match opt {
        Some(_) => println!("Some!"),
        None => println!("None!"),
    };

    println!("{:?}", opt);

    match opt {
        // _ became s, so opt was moved into s
        // if it was Some(_), s would not be moved
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    // opt was moved into s, so it is no longer available
    // Some (ref s) above would have borrowed s instead of moving it
    // println!("{:?}", opt);

    let opt2: Option<String> = Some(String::from("opt string 2"));

    // borrow opt instead of moving it
    // opt became &opt
    // and s became &String
    match &opt2 {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt2);

    // if let instead of single match
    // does not exhaustively check
    // 3u8 is the unsigned 8-bit integer literal 3
    let config_max: Option<u8> = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    if let Some(num) = some_function() {
        println!("Some: {}", num);
    } else {
        println!("None!");
    }
}

// some function that returns an Option<i32>
fn some_function() -> Option<i32> {
    Some(42)
}
