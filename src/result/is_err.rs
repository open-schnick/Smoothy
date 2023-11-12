use crate::{implementation, AssertionBuilder};
use std::fmt::{Debug, Display};

impl<OkValue, ErrValue> AssertionBuilder<Result<OkValue, ErrValue>>
where
    OkValue: Debug,
    ErrValue: Debug,
{
    pub fn is_err(self) -> ErrAsserter<ErrValue> {
        implementation::assert(self.value.is_err(), "Result to be an error", &self.value);

        let value = self.value.err().unwrap();

        ErrAsserter { value }
    }
}

pub struct ErrAsserter<ErrValue> {
    value: ErrValue,
}

// In case the error does implement PartialEq
impl<ErrValue> ErrAsserter<ErrValue>
where
    ErrValue: Debug + PartialEq,
{
    pub fn and_error_equals(self, expected: impl Into<ErrValue>) {
        let expected: ErrValue = expected.into();
        implementation::assert_equals(self.value, expected);
    }
}

// In case the error does not implement PartialEq
impl<ErrValue> ErrAsserter<ErrValue>
where
    ErrValue: Debug + Display,
{
    // TODO: think about this Into. Is AsRef better?
    pub fn and_error_as_string_equals(self, expected: impl Into<String>) {
        let expected: String = expected.into();
        implementation::assert_equals(self.value.to_string(), expected);
    }
}
