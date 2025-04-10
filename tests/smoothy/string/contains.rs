use crate::failing_assertion;
use smoothy::{assert_that, StringAssertion};

#[allow(clippy::unnecessary_to_owned)]
#[test]
fn succeeds_with_str() {
    assert_that("Hello World")
        .contains("Hello")
        .and()
        .contains("World".to_string());
}

#[allow(clippy::unnecessary_to_owned)]
#[test]
fn succeeds_with_string() {
    assert_that("Hello World".to_string())
        .contains("Hello")
        .and()
        .contains("World".to_string());
}

#[test]
fn fails() {
    failing_assertion!({
        assert_that("Hello World").contains("BlaFasel");
    });
}
