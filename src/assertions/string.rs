use crate::{implementation, private, Asserter};

/// Specifies various assertions on [`String`]. Implemented on [`Asserter`]
///
/// This trait is sealed and cannot be implemented outside Smoothy.
pub trait StringAssertion<StringLike>: private::Sealed
where
    StringLike: AsRef<str>,
{
    /// Asserts that the value contains the pattern
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that("Hello World")
    ///     .contains("Hello")
    ///     .and()
    ///     .contains("World");
    /// ```
    ///
    /// # Panics
    /// When the value does not contain the pattern
    #[track_caller]
    fn contains(self, string: impl AsRef<str>) -> Asserter<StringLike>;

    /// Asserts that the value is matching the regex
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// # use regex::Regex;
    /// #
    /// assert_that("I categorically deny having triskaidekaphobia.")
    ///     .matches(&Regex::new(r"\b\w{13}\b").unwrap());
    /// ```
    ///
    /// # Panics
    /// When the value does not match the regex
    #[cfg(feature = "regex")]
    #[cfg_attr(docsrs, doc(cfg(feature = "regex")))]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn matches(self, regex: &regex::Regex) -> Asserter<StringLike>;

    /// Asserts that the value starts with the pattern
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that("Hello World").starts_with("Hello");
    /// ```
    ///
    /// # Panics
    /// When the value does not start with the pattern
    #[track_caller]
    fn starts_with(self, string: impl AsRef<str>) -> Asserter<StringLike>;
}

impl<StringLike> StringAssertion<StringLike> for Asserter<StringLike>
where
    StringLike: AsRef<str>,
{
    fn contains(self, expected: impl AsRef<str>) -> Self {
        let actual = self.value.as_ref();

        implementation::assert(
            actual.contains(expected.as_ref()),
            actual,
            "to contain",
            expected.as_ref(),
        );

        self
    }

    #[cfg(feature = "regex")]
    fn matches(self, regex: &regex::Regex) -> Self {
        let actual = self.value.as_ref();

        implementation::assert(
            regex.is_match(actual),
            actual,
            "to be matched by",
            regex.to_string(),
        );

        self
    }

    fn starts_with(self, expected: impl AsRef<str>) -> Self {
        let actual = self.value.as_ref();

        implementation::assert(
            actual.starts_with(expected.as_ref()),
            actual,
            "to start with",
            expected.as_ref(),
        );

        self
    }
}
