use crate::{implementation, private, AssertionConnector, BasicAsserter};

/// Specifies various assertions on values that can be converted to a boolean. Implemented on [`BasicAsserter`]
///
/// This trait is sealed and cannot be implemented outside Smoothy.
pub trait BooleanAssertion<IntoBoolean>: private::Sealed
where
    IntoBoolean: Into<bool>,
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
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_true(self) -> AssertionConnector<bool>;

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
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_false(self) -> AssertionConnector<bool>;
}

impl<IntoBoolean> BooleanAssertion<IntoBoolean> for BasicAsserter<IntoBoolean>
where
    IntoBoolean: Into<bool>,
{
    fn is_true(self) -> AssertionConnector<bool> {
        let actual = self.value.into();

        implementation::assert(actual, "Value is true", actual);

        AssertionConnector { value: actual }
    }

    fn is_false(self) -> AssertionConnector<bool> {
        let actual = self.value.into();

        implementation::assert(!actual, "Value is false", actual);

        AssertionConnector { value: actual }
    }
}
