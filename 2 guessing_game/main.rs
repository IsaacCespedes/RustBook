// Cargo.toml has notes on semantic versioning
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// error handling via types
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // remember if statements do not wrap their expressions in parens in rust
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");
    // i32 is the default integer type
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    // infinite loop
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // returns io::Result (Ok or Err)
            .expect("Failed to read line"); // unwraps ok, panics / crashes on Err

        // here, u32 is the type annotation
        // it could have been i32, but we want to avoid negative numbers
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        // shadowing (overwriting) the same name with `let`
        // instead of a u32, we can use a custom type to handle errors
        let guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
