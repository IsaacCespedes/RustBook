// Each file in the tests directory is a separate crate
use automated_tests::adder;

// #[cfg (test)] is not needed in integration tests
// cargo test --test <file name without extension> to run a single file

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
