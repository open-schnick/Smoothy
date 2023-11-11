use assertion_builder::AssertionBuilder;

mod assertion_builder;
mod assertions;

pub fn assert_that<AssertedType>(value: AssertedType) -> AssertionBuilder<AssertedType> {
    AssertionBuilder { value }
}
