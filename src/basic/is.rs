use crate::{implementation, AssertionBuilder};

impl<AssertedType> AssertionBuilder<AssertedType>
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
    pub fn is(self, expected: AssertedType) {
        implementation::assert_equals(self.value, expected);
    }
}
