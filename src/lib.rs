mod basic;
mod implementation;
mod result;

pub struct AssertionBuilder<AssertedType> {
    pub(crate) value: AssertedType,
}

pub fn assert_that<AssertedType>(value: AssertedType) -> AssertionBuilder<AssertedType> {
    AssertionBuilder { value }
}
