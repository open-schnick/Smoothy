use crate::{implementation, BasicAsserter};

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    /// Asserts that the assertable is equal to the expected value.
    ///
    /// This is done by transforming the expected-value to a instance of `AssertedType` by using the [`TryInto`]-trait
    /// and then comparing both values with [`PartialEq`]
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// assert_that(42u8).try_into_equals(42i8);
    /// ```
    ///
    /// # Panics
    /// When the transformation fails or the values are not matching according to [`PartialEq`]
    pub fn try_into_equals<T>(self, expected: T)
    where
        T: TryInto<AssertedType>,
        <T as TryInto<AssertedType>>::Error: std::fmt::Debug,
    {
        let conversion_result: Result<AssertedType, _> = expected.try_into();

        implementation::assert(
            conversion_result.is_ok(),
            "TryInto conversion succeeds",
            &conversion_result,
        );

        #[allow(clippy::unwrap_used)]
        let expected = conversion_result.unwrap();

        implementation::assert_equals(self.value, expected);
    }
}
