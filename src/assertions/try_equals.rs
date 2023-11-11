use crate::assertion_builder::AssertionBuilder;

impl<AssertedType> AssertionBuilder<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    // IDEA: can one combine try_equals with equals?
    pub fn try_equals<T>(&self, expected: T)
    where
        T: TryInto<AssertedType>,
        <T as TryInto<AssertedType>>::Error: std::fmt::Debug,
    {
        use pretty_assertions::assert_eq;

        let expected: AssertedType = expected.try_into().unwrap();
        assert_eq!(self.value, expected);
    }
}
