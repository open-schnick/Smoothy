use smoothy::*;

mod assert_option {
    use super::*;

    #[test]
    fn succeeds() {
        let option: Option<String> = Some(String::new());

        assert_that(option).is_some();
    }

    #[test]
    #[should_panic = "assertion failed: `(Option is Some)`"]
    fn fails() {
        let option: Option<()> = None;

        assert_that(option).is_some();
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
