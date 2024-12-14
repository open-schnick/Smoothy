use crate::{implementation, BasicAsserter};

/// Specifies various assertions on [`Result`]. Implemented on [`BasicAsserter`]
pub trait ResultAsserter<OkValue, ErrValue> {
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
    fn is_ok(self) -> OkAsserter<OkValue>;

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
    fn is_err(self) -> ErrAsserter<ErrValue>;
}

impl<OkValue, ErrValue> ResultAsserter<OkValue, ErrValue>
    for BasicAsserter<Result<OkValue, ErrValue>>
where
    OkValue: std::fmt::Debug,
    ErrValue: std::fmt::Debug,
{
    fn is_ok(self) -> OkAsserter<OkValue> {
        implementation::assert(self.value.is_ok(), "Result is Ok", &self.value);

        #[allow(clippy::unwrap_used)]
        let value = self.value.unwrap();

        OkAsserter { value }
    }

    fn is_err(self) -> ErrAsserter<ErrValue> {
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
