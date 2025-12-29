use crate::{implementation, private, Asserter};
use std::fmt::{self, Debug};

/// Specifies various assertions on [`Result`]. Implemented on [`Asserter`]
///
/// This trait is sealed and cannot be implemented outside Smoothy.
pub trait ResultAssertion<OkValue, ErrValue>: private::Sealed {
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
    fn is_ok(self) -> OkAsserter<OkValue>
    where
        ErrValue: Debug;

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
    fn is_err(self) -> ErrAsserter<ErrValue>
    where
        OkValue: Debug;
}

struct ErrWrapper<'a, E>(&'a E);

impl<E: Debug> Debug for ErrWrapper<'_, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Err({:?})", self.0)
    }
}

struct OkWrapper<'a, O>(&'a O);

impl<O: Debug> Debug for OkWrapper<'_, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ok({:?})", self.0)
    }
}

impl<OkValue, ErrValue> ResultAssertion<OkValue, ErrValue> for Asserter<Result<OkValue, ErrValue>> {
    fn is_ok(self) -> OkAsserter<OkValue>
    where
        ErrValue: Debug,
    {
        if let Err(ref e) = self.value {
            implementation::assert_no_expected(false, ErrWrapper(e), "to be Ok");
        }

        #[allow(clippy::unwrap_used)]
        let value = self.value.unwrap();

        OkAsserter { value }
    }

    fn is_err(self) -> ErrAsserter<ErrValue>
    where
        OkValue: Debug,
    {
        if let Ok(ref v) = self.value {
            implementation::assert_no_expected(false, OkWrapper(v), "to be Err");
        }

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
    /// let asserter: Asserter<String> = assert_that(result).is_err().and_error();
    /// // further assertions
    /// asserter.equals("Hello World!");
    /// ```
    #[track_caller]
    #[must_use = "Transforming the asserted value does not assert anything"]
    pub fn and_error(self) -> Asserter<ErrValue> {
        Asserter { value: self.value }
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
    /// let asserter: Asserter<String> = assert_that(result).is_ok().and_value();
    /// // further assertions
    /// asserter.equals("Hello World!");
    /// ```
    #[track_caller]
    #[must_use = "Transforming the asserted value does not assert anything"]
    pub fn and_value(self) -> Asserter<OkValue> {
        Asserter { value: self.value }
    }
}
