// The parent directory - "tests" - which is at the top
// level of our project directory, next to "src" is a specially
// named directory that Cargo knows to look for integration
// tests

// This directory can have as many "intgeration" test files
// as we wish and Cargo will compile each test file into an
// individual crate.

mod common;
use testing::add_two;

// Since each file is an individual crate, we are required to
// bring into scope any library that is required by the test.
// Hence, the "use" statement on line 10

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2))
}
