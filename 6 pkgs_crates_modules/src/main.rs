// Cargo provides workspaces
// for large projects comprising a set of interrelated packages that evolve together

// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

// A package can contain multiple binary crates and optionally one library crate

// A package is a bundle of one or more crates that provides a set of functionality.
// A package contains a Cargo.toml file that describes how to build those crates.

// A package must contain at least one crate, whether that’s a library or binary crate.

// A package can have multiple binary crates by placing files in the src/bin directory:
// each file will be a separate binary crate.

// A crate is the smallest amount of code that the Rust compiler considers at a time
// A crate can come in one of two forms: a binary crate or a library crate.
// most of the time a crate refers to a library crate
// src/lib.rs is the crate root of a library crate with the same name as the package
// src/main.rs is the crate root of a binary crate with the same name as the package

// module paths can take two forms:

// An absolute path is the full path starting from a crate root;
// for code from an external crate, the absolute path begins with the crate name,
// and for code from the current crate, it starts with the literal `crate`.

// A relative path starts from the current module and uses self, super,
// or an identifier in the current module.

mod inline_garden {
    pub mod plant {
        pub fn water() {}
    }
}

// found in src/file_garden.rs
// Rust looks in a file with the same name as the module
mod file_garden;

// found in src/folder_garden/mod.rs
// folder based module is an older style
mod folder_garden;

use folder_garden::vegetables::Cauliflower;

use folder_garden::Asparagus;

fn main() {
    println!("Hello gardens!");
    let cauliflower = Cauliflower {};
    let asparagus = Asparagus {};
}
