use smoothy::{assert_that, EqualityAssertion};

mod succeeds {
    use super::*;

    #[test]
    fn with_numbers() {
        assert_that(21u8).is(21);
    }

    #[test]
    fn with_strings() {
        assert_that(String::from("Hello World!")).is(String::from("Hello World!"));
    }

    #[test]
    fn with_string_slices() {
        assert_that("Hello World!").is("Hello World!");
    }
}

mod fails {
    use super::*;

    #[test]
    #[should_panic = "assertion failed: `(left == right)`"]
    fn with_not_matching_values() {
        assert_that(21u8).is(12);
    }
}
