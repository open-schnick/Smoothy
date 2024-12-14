use regex::Regex;
use smoothy::{assert_that, AssertionConnector, StringAssertion};

#[test]
fn succeeds() {
    let _connector: AssertionConnector<&str> =
        assert_that("I categorically deny having triskaidekaphobia.")
            .is_matching(&Regex::new(r"\b\w{13}\b").unwrap());
}

#[allow(clippy::unnecessary_to_owned)]
#[test]
fn succeeds_with_string() {
    let _connector: AssertionConnector<String> =
        assert_that("I categorically deny having triskaidekaphobia.".to_string())
            .is_matching(&Regex::new(r"\b\w{13}\b").unwrap());
}

#[test]
#[should_panic = "assertion failed: `(Value is matching regex)`\n           found:  \"I deny having fun.\""]
fn fails() {
    assert_that("I deny having fun.".to_string()).is_matching(&Regex::new(r"\b\w{13}\b").unwrap());
}
