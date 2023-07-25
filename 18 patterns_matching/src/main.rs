fn main() {
    // while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop() is the pattern
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // if let
    // v is the pattern
    let v = Some(0u8);
    if let Some(0) = v {
        println!("zero");
    } else {
        println!("not zero");
    }

    // for loop
    // (index, value) is the pattern
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let
    // x is the pattern
    // this pattern is irrefutable
    // because it matches any value
    let x = 5;

    // In the expression if let Some(x) = a_value, then Some(x) is refutable.
    // If the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.
    // In the expression let &[x, ..] = a_slice, then &[x, ..] is refutable.
    // If the value in the a_slice variable has zero elements, the &[x, ..] pattern will not match.

    // let Some(x) = Some(5); // error: requires irrefutable pattern, to cover every possible value

    // Function parameters, let statements, and for loops can only accept irrefutable patterns
    // if let and while let expressions accept refutable and irrefutable patterns

    // (x, y, z) is the pattern
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // function parameters
    /// (and closure parameters)
    let point = (3, 5);
    print_coordinates(&point);

    // matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        5 | 6 | 7 => println!("five, six, or seven"),
        8..=12 => println!("eight to twelve"),
        _ => println!("anything"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Matched, y = 5
    // at the end: x = Some(5), y = 10

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    // same as let Point { x: x, y: y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // destructuring enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // destructuring nested structs and enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, and value {v}",)
        }
        _ => (),
    }

    // destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);

    // ignoring values in a pattern
    bar(3, 4);

    // ignoring parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // unused variables starting with an underscore
    // this wil bind the variable to the value (and trigger move semantics)
    // an underscore alone will  not bind to the value
    let _x = 5;
    let _ = 5;

    // ignoring remaining parts of a value with ..
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 2, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // (.., second, ..)  is not possible
        // because it is not clear which values should be ignored
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // extra conditionals with match guards
    // the compiler does not check for exhaustiveness with match guards
    // match guards allow for testing outer variables

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {x}"),
        Some(x) => println!("{x}"),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // equivalent to (4 | 5 | 6) if y
        _ => println!("no"),
    }

    // @ bindings
    // @ bindings allow for testing a value and saving it in a variable
    // the syntax is let <pattern> = <expression>
    // the @ operator must be used in conjunction with a pattern
    // the @ operator can be used multiple times in a single pattern

    enum Message3 {
        Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {id}"),
    }
}

// function parameters
// x is the pattern
fn foo(x: i32) {
    // code goes here
}

fn bar(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

// &(x, y) is the pattern
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
