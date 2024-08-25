use crate::{implementation, AssertionConnector, BasicAsserter};

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    // NOTE: the type inference for {integers} is bad as i32 does not implement Into<u16>
    /// Asserts that the assertable is equal to the expected value.
    ///
    /// This is done by transforming the expected-value to a instance of `AssertedType` by using the [Into]-trait
    /// and then comparing both values with [`PartialEq`]
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// assert_that(String::from("Hello World!")).equals("Hello World!");
    /// ```
    ///
    /// # Panics
    /// When the values are not matching according to [`PartialEq`]
    #[track_caller]
    pub fn equals(self, expected: impl Into<AssertedType>) -> AssertionConnector<AssertedType> {
        let expected: AssertedType = expected.into();
        implementation::assert_ref_equals(&self.value, expected);
        AssertionConnector { value: self.value }
    }
}
