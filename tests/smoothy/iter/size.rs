use smoothy::{assert_that, EqualityAssertion, IteratorAssertion};

#[test]
fn size_matches() {
    assert_that([1, 2, 3]).size().is(3);
}

#[test]
#[should_panic = "assertion failed: `(left == right)`"]
fn size_does_not_match() {
    assert_that([1, 2, 3]).size().is(42);
}
