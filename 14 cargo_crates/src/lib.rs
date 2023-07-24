//! # My Crate
//! This kind of comment generates docs for a crate or module

// the following comments are generated for the function
// re-exports are generated in the docs also, like these:
// pub use self::kinds::PrimaryColor;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
