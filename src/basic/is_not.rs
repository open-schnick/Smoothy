use crate::{implementation, AssertionBuilder};

impl<AssertedType> AssertionBuilder<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    /// Asserts that the assertable is *not* equal to the expected value while having the same type.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// assert_that("Hello World!").is_not("Hello There!");
    /// ```
    ///
    /// # Panics
    /// When the values are matching.
    pub fn is_not(self, expected: AssertedType) {
        implementation::assert_not_equals(self.value, expected);
    }
}
