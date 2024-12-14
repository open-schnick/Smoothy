use smoothy::{assert_that, StringAssertion};

#[allow(clippy::unnecessary_to_owned)]
#[test]
fn succeeds_with_str() {
    assert_that("Hello World")
        .contains_string("Hello")
        .and()
        .contains_string("World".to_string());
}

#[allow(clippy::unnecessary_to_owned)]
#[test]
fn succeeds_with_string() {
    assert_that("Hello World".to_string())
        .contains_string("Hello")
        .and()
        .contains_string("World".to_string());
}

#[test]
#[should_panic = "assertion failed: `(Value contains string)`\n           found:  \"Hello World\""]
fn fails() {
    assert_that("Hello World").contains_string("BlaFasel");
}
