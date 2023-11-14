use smoothy::assert_that;

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

    #[test]
    #[should_panic = "assertion failed: `(left != right)`"]
    fn with_matching_values() {
        assert_that(12).not_equals(12);
    }
}
