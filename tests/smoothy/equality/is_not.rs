use crate::failing_assertion;
use smoothy::{assert_that, EqualityAssertion};

mod succeeds {
    use super::*;

    #[test]
    fn with_numbers() {
        assert_that(21u8).is_not(42);
    }

    #[test]
    fn with_strings() {
        assert_that(String::from("Hello World!")).is_not(String::from("Hello There!"));
    }

    #[test]
    fn with_string_slices() {
        assert_that("Hello World!").is_not("Hello There!");
    }
}

mod fails {
    use super::*;

    #[test]
    fn with_matching_values() {
        failing_assertion!({
            assert_that(21u8).is_not(21);
        });
    }
}
