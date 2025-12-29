use crate::failing_assertion;
use regex::Regex;
use smoothy::{assert_that, StringAssertion};

#[test]
fn succeeds() {
    assert_that("I categorically deny having triskaidekaphobia.")
        .matches(&Regex::new(r"\b\w{13}\b").unwrap());
}

#[allow(clippy::unnecessary_to_owned)]
#[test]
fn succeeds_with_string() {
    assert_that("I categorically deny having triskaidekaphobia.".to_string())
        .matches(&Regex::new(r"\b\w{13}\b").unwrap());
}

#[test]
fn succeeds_and_can_be_chained() {
    assert_that("I categorically deny having triskaidekaphobia.")
        .matches(&Regex::new(r"\b\w{13}\b").unwrap())
        .and()
        .contains("y hav");
}

#[test]
fn fails() {
    failing_assertion!({
        assert_that("I deny having fun.".to_string()).matches(&Regex::new(r"\b\w{13}\b").unwrap());
    });
}
