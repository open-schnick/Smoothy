use crate::{implementation, BasicAsserter};
use std::fmt::Debug;

impl<SomeValue> BasicAsserter<Option<SomeValue>>
where
    SomeValue: Debug,
{
    /// Asserts that the [Option] is [Some].
    ///
    /// Allows the usage of chained assertions on an option-type (see [`SomeAsserter`]).
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// let option: Option<String> = Some(String::new());
    ///
    /// assert_that(option).is_some();
    /// ```
    ///
    /// # Panics
    /// When the [Option] is [None]
    #[track_caller]
    pub fn is_some(self) -> SomeAsserter<SomeValue> {
        implementation::assert(self.value.is_some(), "Option is Some", &self.value);

        #[allow(clippy::unwrap_used)]
        let value = self.value.unwrap();

        SomeAsserter { value }
    }
}

/// Enables various assertions on [Some]-values
pub struct SomeAsserter<SomeValue> {
    value: SomeValue,
}

impl<SomeValue> SomeAsserter<SomeValue> {
    /// Prepares the [Some] value for further assertions.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::{assert_that, BasicAsserter};
    /// #
    /// let option: Option<String> = Some(String::from("Hello World!"));
    ///
    /// let asserter = assert_that(option).is_some().and_value();
    /// // further assertions
    /// asserter.equals("Hello World!");
    /// ```
    #[track_caller]
    #[must_use = "Transforming the asserted value does not assert anything"]
    pub fn and_value(self) -> BasicAsserter<SomeValue> {
        BasicAsserter { value: self.value }
    }
}
