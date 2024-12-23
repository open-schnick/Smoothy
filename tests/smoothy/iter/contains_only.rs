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
#[should_panic = "assertion failed: `(Iterator contains only the expected items)`\n           found:  [2, 1, 3, 3]"]
fn fails_when_actual_contains_additional_values() {
    assert_that([1, 2, 2, 1, 3, 3]).contains_only([1, 2]);
}

#[test]
#[should_panic = "assertion failed: `(Iterator contains only the expected items)`\n           found:  [3]"]
fn fails_when_expected_contains_additional_values() {
    assert_that([1, 2]).contains_only([1, 2, 3]);
}

#[test]
#[should_panic = "assertion failed: `(Iterator contains only the expected items)`\n           found:  [1]"]
fn fails_when_actual_contains_additional_duplicated_values() {
    assert_that([1, 1, 2]).contains_only([1, 2]);
}
