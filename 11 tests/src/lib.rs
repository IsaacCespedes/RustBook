// tests are run with `cargo test`
// cargo test --help for more options
// cargo test -- --help for more options with the test binary
// tests run in parallel by default
// `cargo test -- --test-threads=1` to run tests serially
// tests are not run when building for release

// only when a test fails does it print its output
// `cargo test -- --show-output` to show output of successful tests too

// you can pass the name (or part of a test name) of any test function to cargo test to run only that test
// e.g. `cargo test it_adds_two`

// `cargo test -- --ignored` to run only ignored tests
// `cargo test -- --include-ignored` to run all tests, including ignored ones

// unit tests go in the src directory in each file with the code that they’re testing.
// convention is to create a module named tests in each file to contain the test functions
// and to annotate the module with cfg(test).

// integration tests go in the tests directory at the top level of the project.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub mod adder {
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "Make this test fail")] // (expected = "Make this test fail") is optional
                                                      // the expected string can be a substring of the panic message
    fn another() {
        panic!("Make this test fail");
    }

    use super::*;

    #[test]
    #[ignore] // ignores (skips) this test
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // For structs and enums that you define yourself,
    // you’ll need to implement PartialEq to assert equality of those types.
    // You’ll also need to implement Debug to print the values when the assertion fails
    #[test]
    fn it_adds_two() {
        use super::adder;
        let placeholder = "placeholder";
        assert_eq!(4, adder::add_two(2));
        assert_ne!(5, adder::add_two(2), "custom message {}", placeholder);
    }

    // tests that use Result<T, E> can use the ? operator in their body
    // but not when asserting that an error should happen
    // for that use the assert!(value.is_err())
    // these tests can't use #[should_panic]
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
