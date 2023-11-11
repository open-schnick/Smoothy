#![allow(dead_code, unused_variables)]
use assertion_builder::AssertionBuilder;

mod assertion_builder;
mod assertions;

fn assert_that<AssertedType>(value: AssertedType) -> AssertionBuilder<AssertedType> {
    AssertionBuilder { value }
}
