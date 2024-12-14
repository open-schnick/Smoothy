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
    #[should_panic = "assertion failed: `(left != right)`"]
    fn with_matching_values() {
        assert_that(21u8).is_not(21);
    }
}
