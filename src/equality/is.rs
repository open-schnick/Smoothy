use crate::{implementation, AssertionConnector, BasicAsserter};

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    /// Asserts that the assertable is equal to the expected value while having the same type.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// assert_that("Hello World!").is("Hello World!");
    /// ```
    ///
    /// # Panics
    /// When the values are not matching.
    #[track_caller]
    pub fn is(self, expected: AssertedType) -> AssertionConnector<AssertedType> {
        implementation::assert_ref_equals(&self.value, expected);
        AssertionConnector { value: self.value }
    }
}
