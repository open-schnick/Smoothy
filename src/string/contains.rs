use crate::{implementation, AssertionConnector, BasicAsserter};

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: AsRef<str>,
{
    /// Asserts that the value contains the pattern
    ///
    /// # Examples
    /// ```
    /// # use smoothy::{assert_that, BasicAsserter};
    /// #
    /// assert_that("Hello World").contains("Hello").and().contains("World");
    /// ```
    ///
    /// # Panics
    /// When the value does not contain the pattern
    pub fn contains(self, pattern: impl AsRef<str>) -> AssertionConnector<AssertedType> {
        let asserted_value = self.value.as_ref();

        implementation::assert(
            asserted_value.contains(pattern.as_ref()),
            "Input contains pattern",
            self.value.as_ref(),
        );

        AssertionConnector { value: self.value }
    }
}
