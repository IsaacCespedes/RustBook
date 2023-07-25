fn main() {
    println!(
        "30 degrees Celsius in Farenheit: {}",
        celsius_to_farenheit(30.0)
    );

    println!(
        "86 degrees Farenheit in Celsius: {}",
        farenheit_to_celsius(80.0)
    );
}

fn celsius_to_farenheit(c: f32) -> i32 {
    (c * (9.0 / 5.0)) as i32 + 32
}

fn farenheit_to_celsius(f: f32) -> i32 {
    ((f - 32.0) * (5.0 / 9.0)) as i32
}
