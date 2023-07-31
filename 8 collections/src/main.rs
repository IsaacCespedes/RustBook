fn main() {
    // Vectors

    let mut v = vec![1, 2, 3, 4, 5];
    // or Vec::new()

    // immutable reference
    // reference is required
    // noncopyable types like String can not move out of vector with indexing
    // vectors can be reallocated for resizability
    let first = v[0]; // panics if out of bounds
                      // v.get(0) // returns an Option<&T>

    // push requires a mutable reference
    // because it may need to reallocate
    // argument is moved when String is pushed
    // v.push(6);

    println!("The first element is: {first}");
    // immutable iterator
    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    // mutable iterator
    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
        println!("{n_ref}");
    }

    // you can't add or remove from a vector while iterating

    // using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // vectors can only store one type
    // but you can use an enum to store multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings

    // Rust has only one string type in the core language,
    // which is the string slice str that is usually seen in its borrowed form &str
    // Strings are provided by Rust’s standard library rather than coded into the core language,
    // is a growable, mutable, owned, UTF-8 encoded string type

    // Rust strings are stored as a vector of bytes (u8)
    // but can be accessed as a string slice (&str)

    let s: String = String::from("string with initial contents");

    // strings are UTF-8 (unicode) encoded
    let hello = String::from("こんにちは");

    // appending a string to a string
    let mut s = String::from("foo");
    s.push_str("bar");

    // appending a char to a string
    let mut s = String::from("lo");
    s.push('l');

    // concatenating strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
                       // + uses the add method, which is implemented by the Add trait for strings
                       // fn add(self, s: &str) -> String

    // format! macro
    // does not take ownership of any of its parameters
    // returns a string
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // indexing strings
    // Rust strings do not support indexing

    // let s1 = String::from("hello");
    // let h = s1[0]; // error: cannot index into a value of type `std::string::String`

    // string slicing is not advised because characters are not all one byte
    // strings can b represented as bytes or chars
    let hi = "Здравствуйте";
    let s = &hi[0..4];

    // iterating over strings
    for c in "Зд".chars() {
        println!("{c}");
    }

    // string slices
    // ("blue");
    // (&String::from("abc")[0..1]);
    // ("  hello there ".trim());
    // ("nice weather".into());

    // strings
    // ("red".to_string());
    // (String::from("hi"));
    // ("rust is fun!".to_owned());
    // ("nice weather".into());
    // (format!("Interpolation {}", "Station"));
    // ("Happy Monday!".to_string().replace("Mon", "Tues"));
    // ("mY sHiFt KeY iS sTiCkY".to_lowercase());

    // Hash Maps

    // keys must be the same type
    // values must be the same type

    // types that implement the Copy trait are copied into the hash map (e.g. i32)
    // Strings are moved and the hash map becomes the owner of the String
    // this is for key and value

    // references are not moved into the hash map
    // they must be valid for the lifetime of the hash map

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 30); // this would overwrite the previous value
    scores.insert(String::from("Yellow"), 50);

    let blue_team_name = String::from("Blue");
    // getreturns a Option<&V> (immutable reference)
    // copied returns a V
    let blue_score = scores.get(&blue_team_name).copied().unwrap_or(0);

    // does not update if key exists
    // entry returns an Entry, which is an enum with Occupied and Vacant variants
    // occupied returns a mutable reference to the value
    scores.entry(String::from("Yellow")).or_insert(150);

    // order is arbitrary
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // "get or create" pattern
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // returns a mutable reference to the value for this key
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // Exercises

    // mode
    let v = vec![1, 2, 3, 4, 5, 5, 5, 6, 7, 8];
    let mode = mode(&v);
    println!("the mode of {:?} is {:?}", v, mode);

    // pig latin
    let s = "apple";
    let s = pig_latin(s);
    println!("apple in pig latin is {s}");

    let s = "first";
    let s = pig_latin(s);
    println!("first in pig latin is {s}");

    // pig latin sentence
    let s = "Hello world, wonderful world.";
    let s = pig_latin_sentence(s);
    println!("Hello world, wonderful world. in pig latin is {s}");
}

// find the mode of a vector
fn mode(v: &Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for n in v {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;

    for (n, count) in &map {
        if count > &max_count {
            mode = **n;
            max_count = *count;
        }
    }

    mode
}

// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
fn pig_latin(s: &str) -> String {
    let mut chars = s.chars();
    let first_char = chars.next().unwrap();
    let rest = chars.as_str();
    let mut result = String::new();

    // check if last char is a punctuation mark
    let last_char = s.chars().last().unwrap();
    let is_punctuation = last_char.is_ascii_punctuation();

    if (first_char == 'a')
        || (first_char == 'e')
        || (first_char == 'i')
        || (first_char == 'o')
        || (first_char == 'u')
    {
        result.push_str(s);
        if !is_punctuation {
            result.push_str("-hay");
        } else {
            result.insert_str(result.len() - 1, "-hay");
        }
    } else {
        result.push_str(rest);

        if !is_punctuation {
            result.push_str("-");
            result.push(first_char);
            result.push_str("ay");
        } else {
            result.insert_str(result.len() - 1, "-");
            result.insert_str(result.len() - 1, &first_char.to_string());
            result.insert_str(result.len() - 1, "ay");
        }
    }
    result
}

// convert sentence to pig latin
fn pig_latin_sentence(s: &str) -> String {
    let mut result = String::new();
    for word in s.split_whitespace() {
        let word = pig_latin(word);
        result.push_str(&word);
        result.push_str(" ");
    }
    result
}
