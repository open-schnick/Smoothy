use crate::{implementation, AssertionConnector, BasicAsserter};

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: AsRef<str>,
{
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
    pub fn contains_string(self, string: impl AsRef<str>) -> AssertionConnector<AssertedType> {
        let asserted_value = self.value.as_ref();

        implementation::assert(
            asserted_value.contains(string.as_ref()),
            "Value contains string",
            asserted_value,
        );

        AssertionConnector { value: self.value }
    }
}
