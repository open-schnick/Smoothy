use crate::{implementation, AssertionBuilder};

impl<AssertedType> AssertionBuilder<AssertedType>
where
    AssertedType: PartialEq + std::fmt::Debug,
{
    // FIXME: the type inference is bad as i32 does not implement Into<u16>
    pub fn not_equals(self, expected: impl Into<AssertedType>) {
        let expected: AssertedType = expected.into();
        implementation::assert_not_equals(self.value, expected)
    }
}
