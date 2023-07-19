use std::io::ErrorKind;

fn main() {
    // panics default to unwinding and cleaning up data first
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
}
