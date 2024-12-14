use crate::{implementation, AssertionConnector, BasicAsserter};

impl<AssertedType> BasicAsserter<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    /// Asserts that the assertable is *not* equal to the expected value.
    ///
    /// This is done by transforming the expected-value to an instance of `AssertedType` by using the [`TryInto`]-trait
    /// and then comparing both values with [`PartialEq`]
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that(42u8).try_into_not_equals(100i8);
    /// ```
    ///
    /// # Panics
    /// When the transformation fails or the values are matching according to [`PartialEq`]
    #[track_caller]
    pub fn try_into_not_equals<T>(self, expected: T) -> AssertionConnector<AssertedType>
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

        implementation::assert_ref_not_equals(&self.value, expected);

        AssertionConnector { value: self.value }
    }
}
