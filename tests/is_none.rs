use smoothy::{assert_that, OptionAssertion};

#[test]
fn succeeds() {
    let option: Option<()> = None;

    assert_that(option).is_none();
}

#[test]
#[should_panic = "assertion failed: `(Option is None)`"]
fn fails() {
    let option: Option<()> = Some(());

    assert_that(option).is_none();
}
