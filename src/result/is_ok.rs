use crate::{implementation, BasicAsserter};
use std::fmt::Debug;

impl<OkValue, ErrValue> BasicAsserter<Result<OkValue, ErrValue>>
where
    OkValue: Debug,
    ErrValue: Debug,
{
    /// Asserts that the [Result] is an [Ok].
    ///
    /// Allows the usage of chained assertions on an ok-value (see [`OkAsserter`]).
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let result: Result<String, ()> = Ok(String::new());
    ///
    /// assert_that(result).is_ok();
    /// ```
    ///
    /// # Panics
    /// When the [Result] is an [Err]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
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

impl<OkValue> OkAsserter<OkValue> {
    /// Prepares the [Ok] value for further assertions.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let result: Result<String, ()> = Ok(String::from("Hello World!"));
    ///
    /// let asserter: BasicAsserter<String> = assert_that(result).is_ok().and_value();
    /// // further assertions
    /// asserter.equals("Hello World!");
    /// ```
    #[track_caller]
    #[must_use = "Transforming the asserted value does not assert anything"]
    pub fn and_value(self) -> BasicAsserter<OkValue> {
        BasicAsserter { value: self.value }
    }
}
