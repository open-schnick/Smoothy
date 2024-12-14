use smoothy::{assert_that, IteratorAssertion};

#[test]
fn succeeds() {
    let vec: Vec<String> = vec![String::from("Hello World!")];
    assert_that(vec).is_not_empty();
}

#[test]
#[should_panic = "assertion failed: `(Iterator is not empty)`\n           found:  None"]
fn fails() {
    let vec: Vec<String> = vec![];
    assert_that(vec).is_not_empty();
}
