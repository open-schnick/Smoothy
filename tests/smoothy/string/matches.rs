use crate::failing_assertion;
use regex::Regex;
use smoothy::{assert_that, AssertionConnector, StringAssertion};

#[test]
fn succeeds() {
    let _connector: AssertionConnector<&str> =
        assert_that("I categorically deny having triskaidekaphobia.")
            .matches(&Regex::new(r"\b\w{13}\b").unwrap());
}

#[allow(clippy::unnecessary_to_owned)]
#[test]
fn succeeds_with_string() {
    let _connector: AssertionConnector<String> =
        assert_that("I categorically deny having triskaidekaphobia.".to_string())
            .matches(&Regex::new(r"\b\w{13}\b").unwrap());
}

#[test]
fn fails() {
    failing_assertion!({
        assert_that("I deny having fun.".to_string()).matches(&Regex::new(r"\b\w{13}\b").unwrap());
    });
}
