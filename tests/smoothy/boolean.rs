use smoothy::{assert_that, BooleanAssertion};

#[cfg(test)]
mod is_true {
    use super::*;

    #[test]
    fn succeeds() {
        assert_that(true).is_true();
    }

    #[test]
    fn succeeds_with_impl_into() {
        assert_that(NewTypeBoolean(true)).is_true();
    }

    #[test]
    #[should_panic = "assertion failed: `(Value is true)`\n           found:  false"]
    fn fails_when_value_is_false() {
        assert_that(false).is_true();
    }
}

#[cfg(test)]
mod is_false {
    use super::*;

    #[test]
    fn succeeds() {
        assert_that(false).is_false();
    }

    #[test]
    fn succeeds_with_impl_into() {
        assert_that(NewTypeBoolean(false)).is_false();
    }

    #[test]
    #[should_panic = "assertion failed: `(Value is false)`\n           found:  true"]
    fn fails_when_value_is_true() {
        assert_that(true).is_false();
    }
}

struct NewTypeBoolean(bool);

impl From<NewTypeBoolean> for bool {
    fn from(value: NewTypeBoolean) -> Self {
        value.0
    }
}
