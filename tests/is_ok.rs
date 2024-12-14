use smoothy::{assert_that, EqualityAssertion};

mod assert_result {
    use super::*;

    #[test]
    fn is_ok_succeeds() {
        let result: Result<String, ()> = Ok(String::new());

        assert_that(result).is_ok();
    }

    #[test]
    #[should_panic = "assertion failed: `(Result is Ok)`\n           found:  Err(())"]
    fn is_ok_fails() {
        let result: Result<String, ()> = Err(());

        assert_that(result).is_ok();
    }
}

mod assert_result_value {
    use super::*;

    #[test]
    fn succeeds() {
        let result: Result<String, ()> = Ok(String::from("Hello There"));

        assert_that(result)
            .is_ok()
            .and_value()
            .equals(String::from("Hello There"));
    }

    #[test]
    fn succeeds_with_trait() {
        let result: Result<String, ()> = Ok(String::from("Hello There"));

        #[allow(clippy::needless_borrow, clippy::needless_borrows_for_generic_args)]
        assert_that(result.clone())
            .is_ok()
            .and_value()
            .equals(&String::from("Hello There"));
        assert_that(result)
            .is_ok()
            .and_value()
            .equals("Hello There");
    }

    #[test]
    fn succeeds_recursively() {
        let result: Result<Result<String, ()>, ()> = Ok(Ok(String::from("Hello There")));

        assert_that(result)
            .is_ok()
            .and_value()
            .is_ok()
            .and_value()
            .equals("Hello There");
    }

    #[test]
    #[should_panic = "assertion failed: `(left == right)`"]
    fn fails() {
        let result: Result<String, ()> = Ok(String::from("Hello There"));

        assert_that(result).is_ok().and_value().equals("yo");
    }
}
