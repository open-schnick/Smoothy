use crate::{implementation, AssertionBuilder};

impl<AssertedType> AssertionBuilder<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    // IDEA: can one combine try_equals with equals?
    pub fn not_try_equals<T>(self, expected: T)
    where
        T: TryInto<AssertedType>,
        <T as TryInto<AssertedType>>::Error: std::fmt::Debug,
    {
        let expected: AssertedType = expected.try_into().unwrap();
        implementation::assert_not_equals(self.value, expected);
    }
}
