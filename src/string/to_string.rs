use crate::BasicAsserter;

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: ToString,
{
    /// Converts the assertable to a string for further assertions
    ///
    /// # Examples
    /// ```
    /// # use smoothy::{assert_that, BasicAsserter};
    /// #
    /// let asserter: BasicAsserter<String> = assert_that(42).to_string();
    /// // further assertions
    /// asserter.equals("42");
    /// ```
    #[track_caller]
    #[must_use = "Transforming the asserted value does not assert anything"]
    pub fn to_string(self) -> BasicAsserter<String> {
        BasicAsserter {
            value: self.value.to_string(),
        }
    }
}
