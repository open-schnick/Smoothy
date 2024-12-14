use crate::{implementation, AssertionConnector, BasicAsserter};
use regex::Regex;

/// Specifies various assertions on [`String`]. Implemented on [`BasicAsserter`]
pub trait StringAssertion<AssertedType> {
    /// Asserts that the value contains the pattern
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that("Hello World")
    ///     .contains_string("Hello")
    ///     .and()
    ///     .contains_string("World");
    /// ```
    ///
    /// # Panics
    /// When the value does not contain the pattern
    #[track_caller]
    fn contains_string(self, string: impl AsRef<str>) -> AssertionConnector<AssertedType>;

    /// Asserts that the value is matching the regex
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// # use regex::Regex;
    /// #
    /// assert_that("I categorically deny having triskaidekaphobia.")
    ///     .is_matching(&Regex::new(r"\b\w{13}\b").unwrap());
    /// ```
    ///
    /// # Panics
    /// When the value does not match the regex
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_matching(self, regex: &Regex) -> AssertionConnector<AssertedType>;

    /// Asserts that the value starts with the pattern
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that("Hello World").starts_with_string("Hello");
    /// ```
    ///
    /// # Panics
    /// When the value does not start with the pattern
    #[track_caller]
    fn starts_with_string(self, string: impl AsRef<str>) -> AssertionConnector<AssertedType>;
}

impl<AssertedType> StringAssertion<AssertedType> for BasicAsserter<AssertedType>
where
    AssertedType: AsRef<str>,
{
    fn contains_string(self, string: impl AsRef<str>) -> AssertionConnector<AssertedType> {
        let asserted_value = self.value.as_ref();

        implementation::assert(
            asserted_value.contains(string.as_ref()),
            "Value contains string",
            asserted_value,
        );

        AssertionConnector { value: self.value }
    }

    fn is_matching(self, regex: &Regex) -> AssertionConnector<AssertedType> {
        let asserted_value = self.value.as_ref();

        implementation::assert(
            regex.is_match(asserted_value),
            "Value is matching regex",
            asserted_value,
        );

        AssertionConnector { value: self.value }
    }

    fn starts_with_string(self, string: impl AsRef<str>) -> AssertionConnector<AssertedType> {
        let asserted_value = self.value.as_ref();

        implementation::assert(
            asserted_value.starts_with(string.as_ref()),
            &format!("Value starts with '{}'", string.as_ref()),
            asserted_value,
        );

        AssertionConnector { value: self.value }
    }
}
