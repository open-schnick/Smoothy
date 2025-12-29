use crate::failing_assertion;
use smoothy::{assert_that, EqualityAssertion, IteratorAssertion};

#[test]
fn connects_with_other_iter_assertions() {
    let vec = ["A", "B", "C"];

    assert_that(vec).is_not_empty().and().second().equals("B");
}

#[test]
fn fails() {
    failing_assertion!({
        let vec: Vec<String> = vec![];
        assert_that(vec).is_not_empty();
    });
}
