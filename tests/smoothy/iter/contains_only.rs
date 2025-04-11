use crate::failing_assertion;
use smoothy::{assert_that, IteratorAssertion};

#[test]
fn ordered() {
    assert_that([1, 2, 3]).contains_only([1, 2, 3]);
}

#[test]
fn unordered() {
    assert_that([1, 2, 3]).contains_only([2, 1, 3]);
}

#[test]
fn duplicates_in_expected() {
    // The difference in order does not matter
    assert_that([1, 1, 2]).contains_only([1, 2, 1]);
}

#[test]
fn can_be_chained_with_connectors() {
    assert_that([1, 2, 3])
        .contains_only([1, 2, 3])
        .and()
        .contains(1);
}

#[test]
fn expected_is_converted_via_into() {
    assert_that(vec![1, 2, 3]).contains_only([1, 2, 3]);
}

#[test]
fn expected_items_are_converted_via_into() {
    assert_that(vec!["A".to_string(), "B".to_string(), "C".to_string()])
        .contains_only(["A", "B", "C"]);
}

#[test]
fn fails_when_actual_contains_additional_values() {
    failing_assertion!({
        assert_that([1, 2, 3]).contains_only([1, 2]);
    });
}

#[test]
fn fails_when_expected_contains_additional_values() {
    failing_assertion!({
        assert_that([1, 2]).contains_only([1, 2, 3]);
    });
}

#[test]
fn fails_when_actual_contains_additional_duplicated_values() {
    failing_assertion!({
        assert_that([1, 1, 2]).contains_only([1, 2]);
    });
}

#[test]
fn fails_when_expected_contains_additional_duplicated_values() {
    failing_assertion!({
        assert_that([1, 2]).contains_only([1, 1, 2]);
    });
}
