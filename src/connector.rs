use crate::BasicAsserter;

/// Basic Connector between two assertions on the same value
///
/// # Examples
/// ```
/// # use smoothy::prelude::*;
/// #
/// assert_that(1).equals(1).and().is(1);
/// ```
pub struct AssertionConnector<AssertedType> {
    pub(crate) value: AssertedType,
}

impl<AssertedType> AssertionConnector<AssertedType> {
    /// Connect two assertions on the same value
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that(1).equals(1).and().is(1);
    /// ```
    #[track_caller]
    #[must_use = "Connecting an assertion without second assertion does nothing"]
    pub fn and(self) -> BasicAsserter<AssertedType> {
        BasicAsserter { value: self.value }
    }
}
