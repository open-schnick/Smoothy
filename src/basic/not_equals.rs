use crate::AssertionBuilder;

impl<AssertedType> AssertionBuilder<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    // FIXME: the type inference is bad as i32 does not implement Into<u16>
    pub fn not_equals(&self, expected: impl Into<AssertedType>) {
        use pretty_assertions::assert_ne;

        let expected: AssertedType = expected.into();
        assert_ne!(self.value, expected);
    }
}
