use crate::{implementation, AssertionConnector, BasicAsserter};

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: Into<bool>,
{
    /// Convenience method for asserting that a value is true
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that(true).is_true();
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that(false).is_true();
    /// ```
    ///
    /// # Panics
    /// When the value is false
    #[allow(clippy::wrong_self_convention)]
    #[track_caller]
    pub fn is_true(self) -> AssertionConnector<bool> {
        let actual = self.value.into();

        implementation::assert(actual, "Value is true", actual);

        AssertionConnector { value: actual }
    }

    /// Convenience method for asserting that a value is false
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that(false).is_false();
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that(true).is_false();
    /// ```
    ///
    /// # Panics
    /// When the value is true
    #[allow(clippy::wrong_self_convention)]
    #[track_caller]
    pub fn is_false(self) -> AssertionConnector<bool> {
        let actual = self.value.into();

        implementation::assert(!actual, "Value is false", actual);

        AssertionConnector { value: actual }
    }
}
