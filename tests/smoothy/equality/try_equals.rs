use crate::failing_assertion;
use smoothy::{assert_that, EqualityAssertion};

mod succeeds {
    use super::*;

    #[test]
    fn with_numbers() {
        assert_that(42u8).try_into_equals(42i8);
    }
}

mod fails {
    use super::*;

    #[test]
    fn with_conversion_error() {
        failing_assertion!({
            assert_that(42u8).try_into_equals(-42i8);
        });
    }

    #[test]
    #[should_panic = "assertion failed: `(left == right)`"]
    fn with_numbers() {
        assert_that(42u8).try_into_equals(100i8);
    }
}
