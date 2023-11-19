use crate::{implementation, BasicAsserter};
use std::fmt::Debug;

impl<OkValue, ErrValue> BasicAsserter<Result<OkValue, ErrValue>>
where
    OkValue: Debug + PartialEq,
    ErrValue: Debug + PartialEq,
{
    /// Asserts that the [Result] is an [Ok].
    ///
    /// Allows the usage of chained assertions on an ok-value (see [`OkAsserter`]).
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// let result: Result<String, ()> = Ok(String::new());
    ///
    /// assert_that(result).is_ok();
    /// ```
    ///
    /// # Panics
    /// When the [Result] is an [Err]
    pub fn is_ok(self) -> OkAsserter<OkValue> {
        implementation::assert(self.value.is_ok(), "Result is Ok", &self.value);

        #[allow(clippy::unwrap_used)]
        let value = self.value.unwrap();

        OkAsserter { value }
    }
}

/// Enables various assertions on [Ok]-values
pub struct OkAsserter<OkValue> {
    value: OkValue,
}

impl<OkValue> OkAsserter<OkValue>
where
    OkValue: Debug + PartialEq,
{
    /// Asserts that the value of the [Ok] is equal to the expected value
    ///
    /// This is done by transforming the expected-value to a instance of `OkValue` by using the [Into]-trait
    /// and then comparing both values with [`PartialEq`]
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// let result: Result<String, ()> = Ok(String::from("Hello There"));
    ///
    /// assert_that(result).is_ok().and_value_equals("Hello There");
    /// ```
    ///
    /// # Panics
    /// When the values are not matching according to [`PartialEq`]
    pub fn and_value_equals(self, expected: impl Into<OkValue>) {
        let expected: OkValue = expected.into();
        implementation::assert_equals(self.value, expected);
    }
}
