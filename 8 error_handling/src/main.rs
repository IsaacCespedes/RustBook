use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

// main can return a Result<T, E> as well
// Result<(), Box<dyn Error>> is a common return type for main
// main can return any type that implements the Termination trait
fn main() {
    // panics (for unrecoverable errors) default to unwinding and cleaning up data first
    // if you want to immediately abort, use panic = 'abort' in Cargo.toml
    // the OS will then have to clean up memory, but this keeps the binary smaller

    // a panic is what occurs when reading array out of bounds
    // the RUST_BACKTRACE env var can be set to get a backtrace
    // panic!("program crash);
    use std::fs::File;

    // for recoverable errors, use Result<T, E>
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // the return type of fs::File::open is Result<T, E>

    let greeting_file_result = File::open("hello.txt");

    // handling files with match
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // handling files with closures and unwrap_or_else
    let greeting_file2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// the ? operator returns a value or errors, coverting to the calling function's return type
// it can only be used in functions that return Result or Option
// similar to Result,  with Options, if the value is None, the None will be returned early from the function at that point.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
