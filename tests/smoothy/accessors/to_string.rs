use smoothy::{assert_that, Asserter, EqualityAssertion};
use utils::TestStruct;

#[test]
fn succeeds_with_string() {
    let asserter: Asserter<String> = assert_that(String::from("Hello World!")).to_string();
    asserter.equals("Hello World!");
}

#[test]
fn succeeds_with_str() {
    let asserter: Asserter<String> = assert_that("Hello World!").to_string();
    asserter.equals("Hello World!");
}

#[test]
fn succeeds_with_number() {
    let asserter: Asserter<String> = assert_that(42).to_string();
    asserter.equals("42");
}

#[test]
fn succeeds_with_struct() {
    let asserter: Asserter<String> = assert_that(TestStruct).to_string();
    asserter.equals("TestStruct");
}

mod utils {
    use std::fmt::Display;
    pub struct TestStruct;

    impl Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestStruct")
        }
    }
}
