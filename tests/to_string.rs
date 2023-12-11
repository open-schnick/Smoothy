use smoothy::*;
use utils::TestStruct;

#[test]
fn succeeds_with_string() {
    let asserter: BasicAsserter<String> = assert_that(String::from("Hello World!")).to_string();
    asserter.equals("Hello World!");
}

#[test]
fn succeeds_with_str() {
    let asserter: BasicAsserter<String> = assert_that("Hello World!").to_string();
    asserter.equals("Hello World!");
}

#[test]
fn succeeds_with_number() {
    let asserter: BasicAsserter<String> = assert_that(42).to_string();
    asserter.equals("42");
}

#[test]
fn succeeds_with_struct() {
    let asserter: BasicAsserter<String> = assert_that(TestStruct).to_string();
    asserter.equals("TestStruct");
}

mod utils {
    pub struct TestStruct;

    impl ToString for TestStruct {
        fn to_string(&self) -> String {
            String::from("TestStruct")
        }
    }
}
