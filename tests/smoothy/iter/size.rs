use crate::failing_assertion;
use smoothy::{assert_that, EqualityAssertion, IteratorAssertion};

#[test]
fn size_matches() {
    assert_that([1, 2, 3]).size().is(3);
}

#[test]
fn size_does_not_match() {
    failing_assertion!({
        assert_that([1, 2, 3]).size().is(42);
    });
}
