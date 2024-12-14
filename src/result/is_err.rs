use crate::{implementation, BasicAsserter};
use std::fmt::Debug;

impl<OkValue, ErrValue> BasicAsserter<Result<OkValue, ErrValue>>
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
    /// # use smoothy::prelude::*;
    /// #
    /// let result: Result<(), String> = Err(String::new());
    ///
    /// assert_that(result).is_err();
    /// ```
    ///
    /// # Panics
    /// When the [Result] is an [Ok]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_err(self) -> ErrAsserter<ErrValue> {
        implementation::assert(self.value.is_err(), "Result is Err", &self.value);

        #[allow(clippy::unwrap_used)]
        let value = self.value.err().unwrap();

        ErrAsserter { value }
    }
}

/// Enables various assertions on [Err]-values
pub struct ErrAsserter<ErrValue> {
    value: ErrValue,
}

impl<ErrValue> ErrAsserter<ErrValue> {
    /// Prepares the [Err] value for further assertions.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let result: Result<(), String> = Err(String::from("Hello World!"));
    ///
    /// let asserter: BasicAsserter<String> = assert_that(result).is_err().and_error();
    /// // further assertions
    /// asserter.equals("Hello World!");
    /// ```
    #[track_caller]
    #[must_use = "Transforming the asserted value does not assert anything"]
    pub fn and_error(self) -> BasicAsserter<ErrValue> {
        BasicAsserter { value: self.value }
    }
}
