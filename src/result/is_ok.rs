use crate::{implementation, AssertionBuilder};

impl<OkValue, ErrValue> AssertionBuilder<Result<OkValue, ErrValue>>
where
    OkValue: std::fmt::Debug + PartialEq,
    ErrValue: std::fmt::Debug + PartialEq,
{
    pub fn is_ok(self) -> OkAsserter<OkValue> {
        implementation::assert(self.value.is_ok(), "Result to be Ok", &self.value);

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
        let expected: OkValue = expected.into();
        implementation::assert_equals(self.value, expected);
    }
}
