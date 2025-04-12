use smoothy::{assert_that, EqualityAssertion};

mod succeeds {
    use super::*;

    #[test]
    fn with_numbers() {
        assert_that(12).not_equals(21);
    }

    #[test]
    fn with_strings() {
        assert_that(String::from("Yo")).not_equals("Hello There!");
    }
}

mod fails {
    use super::*;
    use crate::failing_assertion;

    #[test]
    fn with_matching_values() {
        failing_assertion!({
            assert_that(12).not_equals(12);
        });
    }
}
