use crate::AssertionBuilder;

impl<AssertedType> AssertionBuilder<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    // FIXME: the type inference is bad as i32 does not implement Into<u16>
    pub fn equals(&self, expected: impl Into<AssertedType>) {
        use pretty_assertions::assert_eq;

        let expected: AssertedType = expected.into();
        assert_eq!(self.value, expected);
    }
}
