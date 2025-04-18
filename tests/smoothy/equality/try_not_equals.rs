use crate::failing_assertion;
use smoothy::{assert_that, EqualityAssertion};

mod succeeds {
    use super::*;

    #[test]
    fn with_numbers() {
        assert_that(42u8).try_into_not_equals(100i8);
    }
}

mod fails {
    use super::*;

    #[test]
    fn with_conversion_error() {
        failing_assertion!({
            assert_that(42u8).try_into_not_equals(-100i8);
        });
    }

    #[test]
    fn with_numbers() {
        failing_assertion!({
            assert_that(42u8).try_into_not_equals(42i8);
        });
    }
}
