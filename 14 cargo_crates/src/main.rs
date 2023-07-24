// Cargo has two main profiles:
// the dev profile Cargo uses when you run cargo build
// and the release profile Cargo uses when you run cargo build --release.

// default opt levels in Cargo.toml
// [profile.dev]
// opt-level = 0

// [profile.release]
// opt-level = 3

// documentation comments have three slashes: ///
// documentation comments support Markdown notation
// html docs are generated with cargo doc --open
// This command runs the rustdoc tool distributed with Rust
// and puts the generated HTML documentation in the target/doc directory.
// see lib.rs for documentation comments

// other sections that crate authors commonly use in their documentation:
// Panics, Errors, and Safety.

// code blocks in documentation comments are run as tests during cargo test

fn main() {
    println!("Hello, world!");
}
