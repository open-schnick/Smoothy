use crate::{implementation, AssertionBuilder};
use std::fmt::Debug;

impl<OkValue, ErrValue> AssertionBuilder<Result<OkValue, ErrValue>>
where
    OkValue: Debug,
    ErrValue: Debug,
{
    /// Asserts that the [Result] is an [Err].
    ///
    /// Allows the usage of chained assertions on an error-type (see [`ErrAsserter`]).
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// let result: Result<(), String> = Err(String::new());
    ///
    /// assert_that(result).is_err();
    /// ```
    ///
    /// # Panics
    /// When the [Result] is an [Ok]
    pub fn is_err(self) -> ErrAsserter<ErrValue> {
        implementation::assert(self.value.is_err(), "Result to be an error", &self.value);

        let value = self.value.err().unwrap();

        ErrAsserter { value }
    }
}

/// Enables various assertions on [Err]-values
pub struct ErrAsserter<ErrValue> {
    value: ErrValue,
}

impl<ErrValue> ErrAsserter<ErrValue>
where
    ErrValue: Debug + PartialEq,
{
    /// Asserts that the asserted error is equal to the expected error.
    ///
    /// This is done by transforming the expected error to a instance of `ErrValue` by using the [Into]-trait
    /// and then comparing both values with [`PartialEq`].
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// # use std::fmt::Display;
    /// #
    /// #[derive(Debug, PartialEq)]
    /// pub struct ComparableError(String);
    ///
    /// let result: Result<(), ComparableError> = Err(ComparableError(String::from("Hello There")));
    ///
    /// assert_that(result)
    ///     .is_err()
    ///     .and_error_equals(ComparableError(String::from("Hello There")));
    /// ```
    ///
    /// When the asserted error cannot be build but converted one can also use any type that implements [Into] the asserted error
    ///
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// // The error type we want to assert
    /// #[derive(Debug, PartialEq)]
    /// pub struct NotConstructableError(String);
    ///
    /// // The error we can construct
    /// pub struct ConvertableError(pub String);
    ///
    /// /* From-implementation */
    /// # impl From<ConvertableError> for NotConstructableError {
    /// #     fn from(error: ConvertableError) -> Self {
    /// #         NotConstructableError(error.0)
    /// #     }
    /// # }
    ///
    /// let result: Result<(), NotConstructableError> = Err(NotConstructableError(String::from("Hello There")));
    ///
    /// assert_that(result)
    ///     .is_err()
    ///     .and_error_equals(ConvertableError(String::from("Hello There")));
    /// ```
    ///
    /// # Panics
    /// When the errors are not matching according to [`PartialEq`].
    pub fn and_error_equals(self, expected: impl Into<ErrValue>) {
        let expected: ErrValue = expected.into();
        implementation::assert_equals(self.value, expected);
    }
}

impl<ErrValue> ErrAsserter<ErrValue>
where
    ErrValue: Debug + ToString,
{
    /// Asserts that the asserted error message is equal to the expected string.
    ///
    /// This assertion can be used when the error does not implement [`PartialEq`] or the error cannot be constructed.
    ///
    /// The asserted error message is created by calling [`to_string`](ToString::to_string) on the error
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// # use std::fmt::Display;
    /// #
    /// #[derive(Debug)]
    /// pub struct NonComparableError(String);
    /// #
    /// # impl Display for NonComparableError {
    /// #     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    /// #         write!(f, "{}", self.0)
    /// #     }
    /// # }
    ///
    /// let result: Result<(), NonComparableError> =
    /// Err(NonComparableError(String::from("Hello There")));
    ///
    /// assert_that(result)
    ///     .is_err()
    ///     .and_error_to_string_equals(String::from("Hello There"));
    /// ```
    ///
    /// # Panics
    /// When the error message is not equal to the expected error message.
    pub fn and_error_to_string_equals(self, expected: impl AsRef<str>) {
        implementation::assert_equals(self.value.to_string().as_ref(), expected.as_ref());
    }
}
