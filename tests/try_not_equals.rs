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
    #[should_panic = "assertion failed: `(TryInto conversion succeeds)`\n           found:  Err(TryFromIntError(()))"]
    fn with_conversion_error() {
        assert_that(42u8).try_into_not_equals(-100i8);
    }

    #[test]
    #[should_panic = "assertion failed: `(left != right)`"]
    fn with_numbers() {
        assert_that(42u8).try_into_not_equals(42i8);
    }
}
