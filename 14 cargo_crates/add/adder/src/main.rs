// run this package with `cargo run -p adder`
// if add_one required rand 0.8.1 and adder required rand 0.7.3, then
// cargo would build booth versions of rand
// and use the correct one for each crate
// if adder required rand 0.8.1 and add_one required rand 0.8.3, then
// cargo would build 0.8.3 for both crates

use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
