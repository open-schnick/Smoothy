use crate::{implementation, AssertionConnector, BasicAsserter};
use regex::Regex;

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: AsRef<str>,
{
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
    pub fn is_matching(self, regex: &Regex) -> AssertionConnector<AssertedType> {
        let asserted_value = self.value.as_ref();

        implementation::assert(
            regex.is_match(asserted_value),
            "Value is matching regex",
            asserted_value,
        );

        AssertionConnector { value: self.value }
    }
}
