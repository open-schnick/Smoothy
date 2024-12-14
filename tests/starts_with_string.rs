use smoothy::{assert_that, StringAssertion};

#[allow(clippy::unnecessary_to_owned)]
#[test]
fn succeeds_with_str() {
    assert_that("Hello World")
        .starts_with_string("Hello")
        .and()
        .starts_with_string("Hello".to_string());
}

#[allow(clippy::unnecessary_to_owned)]
#[test]
fn succeeds_with_string() {
    assert_that("Hello World".to_string())
        .starts_with_string("Hello")
        .and()
        .starts_with_string("Hello".to_string());
}

#[test]
#[should_panic = "assertion failed: `(Value starts with 'BlaFasel')`\n           found:  \"Hello World\""]
fn fails() {
    assert_that("Hello World").starts_with_string("BlaFasel");
}
