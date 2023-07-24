// Files in subdirectories of the tests directory don’t get compiled as separate crates
// or have sections in the test output
// it will not be run during  `cargo test`

// If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file,
// we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement.
// Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

// This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file.
// Using that structure, integration tests can test the library crate with use to make the important functionality available.
// If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.
