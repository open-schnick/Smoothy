use crate::failing_assertion;
use smoothy::{assert_that, IteratorAssertion};

#[test]
fn succeeds() {
    let vec: Vec<String> = vec![];
    assert_that(vec).is_empty();
}

#[test]
fn fails() {
    failing_assertion!({
        let vec: Vec<String> = vec!["Hello".to_string()];
        assert_that(vec).is_empty();
    });
}
