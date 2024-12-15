use crate::{implementation, AssertionConnector, BasicAsserter};
use std::fmt::Debug;

/// Specifies various equality assertions. Implemented on [`BasicAsserter`]
pub trait EqualityAssertion<AssertedType>
where
    AssertedType: PartialEq + Debug,
{
    // NOTE: the type inference for {integers} is bad as i32 does not implement Into<u16>
    /// Asserts that the assertable is equal to the expected value.
    ///
    /// This is done by transforming the expected-value to an instance of `AssertedType` by using the [Into]-trait
    /// and then comparing both values with [`PartialEq`]
    ///
    /// # Examples
    /// ```
    /// use smoothy::prelude::*;
    ///
    /// assert_that(String::from("Hello World!")).equals("Hello World!");
    /// ```
    ///
    /// # Panics
    /// When the values are not matching according to [`PartialEq`]
    #[track_caller]
    fn equals(self, expected: impl Into<AssertedType>) -> AssertionConnector<AssertedType>;

    // NOTE: the type inference for {integers} is bad as i32 does not implement Into<u16>
    /// Asserts that the assertable is *not* equal to the expected value.
    ///
    /// This is done by transforming the expected-value to an instance of `AssertedType` by using the [Into]-trait
    /// and then comparing both values with [`PartialEq`]
    ///
    /// # Examples
    /// ```
    /// use smoothy::prelude::*;
    ///
    /// assert_that(String::from("Hello World!")).not_equals("Hello There!");
    /// ```
    ///
    /// # Panics
    /// When the values are matching according to [`PartialEq`]
    #[track_caller]
    fn not_equals(self, expected: impl Into<AssertedType>) -> AssertionConnector<AssertedType>;

    /// Asserts that the assertable is equal to the expected value.
    ///
    /// This is done by transforming the expected-value to an instance of `AssertedType` by using the [`TryInto`]-trait
    /// and then comparing both values with [`PartialEq`]
    ///
    /// # Examples
    /// ```
    /// use smoothy::prelude::*;
    ///
    /// assert_that(42u8).try_into_equals(42i8);
    /// ```
    ///
    /// # Panics
    /// When the transformation fails or the values are not matching according to [`PartialEq`]
    #[track_caller]
    fn try_into_equals<T>(self, expected: T) -> AssertionConnector<AssertedType>
    where
        T: TryInto<AssertedType>,
        <T as TryInto<AssertedType>>::Error: Debug;

    /// Asserts that the assertable is *not* equal to the expected value.
    ///
    /// This is done by transforming the expected-value to an instance of `AssertedType` by using the [`TryInto`]-trait
    /// and then comparing both values with [`PartialEq`]
    ///
    /// # Examples
    /// ```
    /// use smoothy::prelude::*;
    ///
    /// assert_that(42u8).try_into_not_equals(100i8);
    /// ```
    ///
    /// # Panics
    /// When the transformation fails or the values are matching according to [`PartialEq`]
    #[track_caller]
    fn try_into_not_equals<T>(self, expected: T) -> AssertionConnector<AssertedType>
    where
        T: TryInto<AssertedType>,
        <T as TryInto<AssertedType>>::Error: Debug;

    /// Asserts that the assertable is equal to the expected value while having the same type.
    ///
    /// # Examples
    /// ```
    /// use smoothy::prelude::*;
    ///
    /// assert_that("Hello World!").is("Hello World!");
    /// ```
    ///
    /// # Panics
    /// When the values are not matching.
    #[track_caller]
    fn is(self, expected: AssertedType) -> AssertionConnector<AssertedType>;

    /// Asserts that the assertable is *not* equal to the expected value while having the same type.
    ///
    /// # Examples
    /// ```
    /// use smoothy::prelude::*;
    ///
    /// assert_that("Hello World!").is_not("Hello There!");
    /// ```
    ///
    /// # Panics
    /// When the values are matching.
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_not(self, expected: AssertedType) -> AssertionConnector<AssertedType>;
}

impl<AssertedType> EqualityAssertion<AssertedType> for BasicAsserter<AssertedType>
where
    AssertedType: PartialEq + Debug,
{
    fn equals(self, expected: impl Into<AssertedType>) -> AssertionConnector<AssertedType> {
        let transformed_expected: AssertedType = expected.into();
        implementation::assert_ref_equals(&self.value, transformed_expected);
        AssertionConnector { value: self.value }
    }

    fn not_equals(self, expected: impl Into<AssertedType>) -> AssertionConnector<AssertedType> {
        let transformed_expected: AssertedType = expected.into();
        implementation::assert_ref_not_equals(&self.value, transformed_expected);
        AssertionConnector { value: self.value }
    }

    fn try_into_equals<T>(self, expected: T) -> AssertionConnector<AssertedType>
    where
        T: TryInto<AssertedType>,
        <T as TryInto<AssertedType>>::Error: Debug,
    {
        let conversion_result: Result<AssertedType, _> = expected.try_into();

        implementation::assert(
            conversion_result.is_ok(),
            "TryInto conversion succeeds",
            &conversion_result,
        );

        #[allow(clippy::unwrap_used)]
        let expected = conversion_result.unwrap();

        implementation::assert_ref_equals(&self.value, expected);

        AssertionConnector { value: self.value }
    }

    fn try_into_not_equals<T>(self, expected: T) -> AssertionConnector<AssertedType>
    where
        T: TryInto<AssertedType>,
        <T as TryInto<AssertedType>>::Error: Debug,
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

    fn is(self, expected: AssertedType) -> AssertionConnector<AssertedType> {
        implementation::assert_ref_equals(&self.value, expected);
        AssertionConnector { value: self.value }
    }

    fn is_not(self, expected: AssertedType) -> AssertionConnector<AssertedType> {
        implementation::assert_ref_not_equals(&self.value, expected);
        AssertionConnector { value: self.value }
    }
}
