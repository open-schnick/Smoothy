use crate::{assert_that, assertion_builder::AssertionBuilder};

impl<OkValue, ErrValue> AssertionBuilder<Result<OkValue, ErrValue>>
where
    OkValue: std::fmt::Debug + PartialEq,
    ErrValue: std::fmt::Debug + PartialEq,
{
    pub fn is_ok(self) -> OkAsserter<OkValue> {
        assert!(
            self.value.is_ok(),
            "Expected value to be Ok but was {:?}",
            self.value
        );

        let value = self.value.unwrap();

        OkAsserter { value }
    }
}

pub struct OkAsserter<OkValue> {
    value: OkValue,
}

impl<OkValue> OkAsserter<OkValue>
where
    OkValue: std::fmt::Debug + PartialEq,
{
    pub fn and_value_equals(self, expected: impl Into<OkValue>) {
        assert_that(self.value).equals(expected);
    }
}
