use crate::{implementation, AssertionConnector, BasicAsserter};

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    /// Asserts that the assertable is *not* equal to the expected value while having the same type.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that("Hello World!").is_not("Hello There!");
    /// ```
    ///
    /// # Panics
    /// When the values are matching.
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_not(self, expected: AssertedType) -> AssertionConnector<AssertedType> {
        implementation::assert_ref_not_equals(&self.value, expected);
        AssertionConnector { value: self.value }
    }
}
