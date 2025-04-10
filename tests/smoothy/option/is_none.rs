use crate::failing_assertion;
use smoothy::{assert_that, OptionAssertion};

#[test]
fn succeeds() {
    let option: Option<()> = None;

    assert_that(option).is_none();
}

#[test]
fn fails() {
    failing_assertion!({
        let option: Option<()> = Some(());

        assert_that(option).is_none();
    });
}
