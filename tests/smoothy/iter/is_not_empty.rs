use smoothy::{assert_that, AssertionConnector, EqualityAssertion, IteratorAssertion};

#[test]
fn succeeds() {
    let vec: Vec<String> = vec![String::from("Hello World!")];
    let _connector: AssertionConnector<Vec<String>> = assert_that(vec).is_not_empty();
}

#[test]
fn connects_with_other_iter_assertions() {
    let vec = ["A", "B", "C"];

    assert_that(vec).is_not_empty().and().second().equals("B");
}

#[test]
#[should_panic = "assertion failed: `(Iterator is not empty)`\n           found:  []"]
fn fails() {
    let vec: Vec<String> = vec![];
    assert_that(vec).is_not_empty();
}
