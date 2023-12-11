use smoothy::assert_that;

#[test]
fn succeeds() {
    let vec: Vec<String> = vec![];
    assert_that(vec).is_empty();
}

#[test]
#[should_panic = "assertion failed: `(Iterator is empty)`\n           found:  Some(\"Hello World!\")"]
fn fails() {
    let vec: Vec<String> = vec![String::from("Hello World!")];
    assert_that(vec).is_empty();
}
