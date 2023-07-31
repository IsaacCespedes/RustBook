const TWO: u32 = 1 + 1; // const has global scope, known at compile time

fn main() {
    println!("constant value: {TWO}");

    // casing convention
    // snake_case for functions and variables
    // PascalCase for types
    // SCREAMING_SNAKE_CASE for constants

    // scalars: ints, floats (f64 is default), bools, chars

    // let keyword can only be used in functioons
    // immutable by default
    // i32 is default integer type
    // (two's complement)

    // examples with different bases
    // Decimal 98_222
    // Hex	0xff
    // Octal	0o77
    // Binary	0b1111_0000
    // Byte (u8 only)	b'A'

    let a = 5;

    // shadowing (redeclaring) a variable
    // works to change a variable's type
    // instead of declaring a new variable
    // (e.g. i32 to string)
    // note: can not be mutable, and must use let keyword
    let a: u32 = a + 1;

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}"); //12
    }

    println!("The value of a in the outer scope is: {a}"); // 6

    let mut b = 5; // mutable variable
    println!("The value of b is: {b}"); // 5
    b = 6;
    println!("The value of b is: {b}"); // 6

    // tuples, destructuring
    let tup = (500, 6.4, 1);

    // unit type, implicit when an expression does not return a value
    let _unit_tup = ();

    // underscore to ignore unused variables
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    // chars are unicode, 4 bytes in size
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {heart_eyed_cat}");

    let _p: bool = true;

    // will panic in debug and wrap around in release
    // let overflow: u8 = 256;

    let q: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of q[2] is: {}", q[2]);

    // zero fill
    let r = [0; 5];
    println!("The value of r[2] is: {}", r[2]);

    print_labeled_measurement(5, 'h');
    another_function(4);
    let value = add_five(5);
    println!("The value of value is: {value}");

    println!(
        "30 degrees Celsius in Farenheit: {}",
        celsius_to_farenheit(30)
    );

    println!(
        "86 degrees Farenheit in Celsius: {}",
        farenheit_to_celsius(80)
    );

    // expressions vs statements
    // statements do not return values
    // let x = (let y = 6); // error
    // let foo = 5; // statement
    let bar = {
        // expression
        let baz = 3;
        baz + 1 // no semicolon
    };
    println!("The value of bar is: {bar}"); // 4

    let number = 3;
    // no casting to bool
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // eval types must match in conditional expressions
    let condition = true;
    let cond_num = if condition { 5 } else { 6 }; // cond_num = 5, note cases both return ints

    println!("The value of number is: {cond_num}");

    // infinite loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break can return a value
        }
    };

    println!("The result is {result}"); // 20

    // loop labeling
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}"); // 0, 1, 2
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}"); // (10, 9), (10, 9), (10)
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); // 2

    // while loop
    let mut loop_num = 5;

    while loop_num > 3 {
        println!("while loooping {loop_num}!"); // 5, 4,
        loop_num -= 1;
    }

    // ranges are more common
    // end range is not inclusive by default
    for number in (1..=3).rev() {
        println!(" reverse looping {number}!"); // 3, 2, 1
    }

    println!("LIFTOFF!!!");

    // for loop
    let elems = [10, 20, 30, 40, 50];

    // for in
    for element in elems {
        println!("the value is: {element}");
    }
}

// In function signatures, you must declare the type of each parameter.
fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// return value - expression, without semicolon
// a semicolon turns an expression into a statement, which will return the unit type ()
fn add_five(value: i32) -> i32 {
    value + 5
}

fn celsius_to_farenheit(c: i32) -> i32 {
    (c * (9 / 5)) + 32
}

fn farenheit_to_celsius(f: i32) -> f32 {
    ((f - 32) * (5 / 9)) as f32
}
