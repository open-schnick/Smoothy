use smoothy::assert_that;

#[test]
fn succeeds() {
    assert_that("Hello World")
        .contains("Hello")
        .and()
        .contains("World");
}

#[allow(clippy::unnecessary_to_owned)]
#[test]
fn succeeds_with_string() {
    assert_that("Hello World".to_string())
        .contains("Hello".to_string())
        .and()
        .contains("World".to_string());
}

#[test]
#[should_panic = "assertion failed: `(Value contains pattern)`\n           found:  \"Hello World\""]
fn fails() {
    assert_that("Hello World").contains("BlaFasel");
}
