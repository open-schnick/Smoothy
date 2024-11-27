use crate::{implementation, AssertionConnector, BasicAsserter};

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: AsRef<str>,
{
    /// Asserts that the value starts with the pattern
    ///
    /// # Examples
    /// ```
    /// # use smoothy::{assert_that, BasicAsserter};
    /// #
    /// assert_that("Hello World").starts_with_string("Hello");
    /// ```
    ///
    /// # Panics
    /// When the value does not start with the pattern
    #[track_caller]
    pub fn starts_with_string(self, string: impl AsRef<str>) -> AssertionConnector<AssertedType> {
        let asserted_value = self.value.as_ref();

        implementation::assert(
            asserted_value.starts_with(string.as_ref()),
            &format!("Value starts with '{}'", string.as_ref()),
            asserted_value,
        );

        AssertionConnector { value: self.value }
    }
}
