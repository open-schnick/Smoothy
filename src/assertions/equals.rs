use crate::assertion_builder::AssertionBuilder;

impl<AssertedType> AssertionBuilder<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    // FIXME: the type inference is bad as i32 does not implement Into<u16>
    pub fn equals(&self, expected: impl Into<AssertedType>) {
        use pretty_assertions::assert_eq;

        let expected: AssertedType = expected.into();
        assert_eq!(self.value, expected);
    }
}

#[cfg(test)]
mod with_numbers {
    use crate::assert_that;

    mod u8 {
        use super::*;

        #[test]
        fn min() {
            assert_that(u8::MIN).equals(0);
        }

        #[test]
        fn max() {
            assert_that(u8::MAX).equals(255);
        }

        #[test]
        fn random_value() {
            let random_value: u8 = 123;
            assert_that(random_value).equals(123);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: u8 = 123;
            assert_that(random_value).equals(42);
        }
    }

    mod u16 {
        use super::*;

        #[test]
        fn min() {
            assert_that(u16::MIN).equals(0u16);
        }

        #[test]
        fn max() {
            assert_that(u16::MAX).equals(65535u16);
        }

        #[test]
        fn random_value() {
            let random_value: u16 = 321;
            assert_that(random_value).equals(321u16);
        }

        #[test]
        fn can_be_compared_to_u8() {
            let random_value: u16 = 123;
            assert_that(random_value).equals(123u8);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: u16 = 123;
            assert_that(random_value).equals(321u16);
        }
    }

    mod u32 {
        use super::*;

        #[test]
        fn min() {
            assert_that(u32::MIN).equals(0u32);
        }

        #[test]
        fn max() {
            assert_that(u32::MAX).equals(4294967295u32);
        }

        #[test]
        fn random_value() {
            let random_value: u32 = 321;
            assert_that(random_value).equals(321u32);
        }

        #[test]
        fn can_be_compared_to_u8() {
            let random_value: u32 = 123;
            assert_that(random_value).equals(123u8);
        }

        #[test]
        fn can_be_compared_to_u16() {
            let random_value: u32 = 123;
            assert_that(random_value).equals(123u16);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: u32 = 123;
            assert_that(random_value).equals(321u32);
        }
    }
}
