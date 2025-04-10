use crate::failing_assertion;
use smoothy::{assert_that, BasicAsserter, EqualityAssertion, OptionAssertion};

mod assert_option {
    use super::*;

    #[test]
    fn succeeds() {
        let option: Option<String> = Some(String::new());

        assert_that(option).is_some();
    }

    #[test]
    fn fails() {
        failing_assertion!({
            let option: Option<()> = None;

            assert_that(option).is_some();
        });
    }
}

mod assert_some {
    use super::*;

    #[test]
    fn transforms_to_basic_asserter() {
        let option: Option<String> = Some(String::new());

        let asserter: BasicAsserter<String> = assert_that(option).is_some().and_value();
        asserter.equals("");
    }
}
