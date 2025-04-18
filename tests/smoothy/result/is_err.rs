use crate::{
    failing_assertion,
    result::is_err::setup::{ComparableError, ConvertableError, NonComparableError},
};
use smoothy::{assert_that, EqualityAssertion, ResultAssertion};
use std::fmt::Display;

mod assert_result {
    use super::*;

    #[test]
    fn is_an_error_succeeds() {
        let result: Result<(), String> = Err(String::new());
        assert_that(result).is_err();
    }

    #[test]
    fn is_an_error_fails() {
        failing_assertion!({
            let result: Result<(), String> = Ok(());
            assert_that(result).is_err();
        });
    }
}

mod assert_error {
    use super::*;

    #[test]
    fn succeeds() {
        let result: Result<(), ComparableError> = Err(ComparableError(String::from("Hello There")));

        assert_that(result)
            .is_err()
            .and_error()
            .equals(ComparableError(String::from("Hello There")));
    }

    #[test]
    fn succeeds_with_another_error_by_trait_conversion() {
        let result: Result<(), ComparableError> = Err(ComparableError(String::from("Hello There")));

        assert_that(result)
            .is_err()
            .and_error()
            .equals(ConvertableError(String::from("Hello There")));
    }

    #[test]
    fn succeeds_recursively() {
        let result: Result<(), Result<(), ComparableError>> =
            Err(Err(ComparableError(String::from("Hello There"))));

        assert_that(result)
            .is_err()
            .and_error()
            .is_err()
            .and_error()
            .equals(ConvertableError(String::from("Hello There")));
    }

    #[test]
    fn fails() {
        failing_assertion!({
            let result: Result<(), ComparableError> =
                Err(ComparableError(String::from("Hello There")));

            assert_that(result)
                .is_err()
                .and_error()
                .equals(ConvertableError(String::from("yo")));
        });
    }
}

mod assert_error_as_string {
    use super::*;

    #[test]
    fn succeeds() {
        let result: Result<(), NonComparableError> =
            Err(NonComparableError(String::from("Hello There")));

        assert_that(result)
            .is_err()
            .and_error()
            .to_string()
            .equals(String::from("Hello There"));
    }

    #[test]
    fn succeeds_with_trait() {
        let result: Result<(), NonComparableError> =
            Err(NonComparableError(String::from("Hello There")));

        assert_that(result)
            .is_err()
            .and_error()
            .to_string()
            .equals("Hello There");
    }

    #[test]
    fn fails() {
        failing_assertion!({
            let result: Result<(), NonComparableError> =
                Err(NonComparableError(String::from("Hello There")));

            assert_that(result)
                .is_err()
                .and_error()
                .to_string()
                .equals("yo");
        });
    }
}

mod setup {
    use super::*;

    #[derive(Debug)]
    pub struct NonComparableError(pub String);

    impl Display for NonComparableError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct ComparableError(pub String);

    impl Display for ComparableError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    pub struct ConvertableError(pub String);

    impl From<ConvertableError> for ComparableError {
        fn from(error: ConvertableError) -> Self {
            Self(error.0)
        }
    }
}
