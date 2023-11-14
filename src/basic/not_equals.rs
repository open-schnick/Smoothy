use crate::{implementation, AssertionBuilder};

impl<AssertedType> AssertionBuilder<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    // FIXME: the type inference for {integers} is bad as i32 does not implement Into<u16>
    /// Asserts that the assertable is *not* equal to the expected value.
    ///
    /// This is done by transforming the expected-value to a instance of `AssertedType` by using the [Into]-trait
    /// and then comparing both values with [`PartialEq`]
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// assert_that(String::from("Hello World!")).not_equals("Hello There!");
    /// ```
    ///
    /// # Panics
    /// When the values are matching according to [`PartialEq`]
    pub fn not_equals(self, expected: impl Into<AssertedType>) {
        let expected: AssertedType = expected.into();
        implementation::assert_not_equals(self.value, expected);
    }
}
