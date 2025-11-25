use crate::{implementation, private, BasicAsserter};
use std::fmt::Debug;

/// Specifies various assertions on [`Option`]. Implemented on [`BasicAsserter`]
///
/// This trait is sealed and cannot be implemented outside Smoothy.
pub trait OptionAssertion<OptionValue>: private::Sealed {
    /// Asserts that the [Option] is [Some].
    ///
    /// Allows the usage of chained assertions on an option-type (see [`SomeAsserter`]).
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let option: Option<String> = Some(String::new());
    ///
    /// assert_that(option).is_some();
    /// ```
    ///
    /// # Panics
    /// When the [Option] is [None]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_some(self) -> SomeAsserter<OptionValue>
    where
        OptionValue: Debug;

    /// Asserts that the [Option] is [None].
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let option: Option<String> = None;
    ///
    /// assert_that(option).is_none();
    /// ```
    ///
    /// # Panics
    /// When the [Option] is [Some]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_none(self)
    where
        OptionValue: Debug;
}

impl<OptionValue> OptionAssertion<OptionValue> for BasicAsserter<Option<OptionValue>> {
    fn is_some(self) -> SomeAsserter<OptionValue>
    where
        OptionValue: Debug,
    {
        implementation::assert_no_expected(self.value.is_some(), &self.value, "to be Some");

        #[allow(clippy::unwrap_used)]
        let value = self.value.unwrap();

        SomeAsserter { value }
    }

    fn is_none(self)
    where
        OptionValue: Debug,
    {
        implementation::assert_no_expected(self.value.is_none(), &self.value, "to be None");
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
    /// # use smoothy::prelude::*;
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
