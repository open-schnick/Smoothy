use smoothy::{assert_that, IteratorAssertion};

#[test]
fn succeeds() {
    let vec: Vec<String> = vec![];
    assert_that(vec).is_empty();
}

#[test]
#[should_panic = "assertion failed: `(Iterator is empty)`\n           found:  [\"\"]"]
fn fails() {
    let vec: Vec<String> = vec![String::default()];
    assert_that(vec).is_empty();
}
