use smoothy::{assert_that, IteratorAssertion};

#[test]
fn ordered() {
    let vec = vec![1, 2, 3, 4, 5];

    assert_that(vec).contains_all([2, 3]);
}

#[test]
fn unordered() {
    let vec = vec![1, 2, 3, 4, 5];

    assert_that(vec).contains_all([5, 2]);
}

#[test]
fn duplicates() {
    assert_that([1, 1, 2, 1, 2, 1, 1]).contains_all([1, 2]);
}

#[test]
fn duplicates_in_expected() {
    assert_that([1, 1, 2, 1, 2, 1, 1]).contains_all([1, 2, 2, 1, 2]);
}

#[test]
fn can_be_chained_with_connectors() {
    let vec = vec![1, 2, 3, 4, 5];

    assert_that(vec)
        .contains_all([2, 1])
        .and()
        .contains_all([3, 4])
        .and()
        .contains_all([4, 5]);
}

#[test]
fn expected_is_converted_via_into() {
    assert_that(vec![1, 2, 3]).contains_all([1, 2, 3]);
}

#[test]
fn expected_items_are_converted_via_into() {
    assert_that(vec!["A".to_string(), "B".to_string(), "C".to_string()])
        .contains_all(["A", "B", "C"]);
}

#[test]
#[should_panic = "assertion failed: `(Iterator contains items)`\n           found:  [4, 420]"]
fn fails_when_expected_contains_a_value_not_in_actual() {
    assert_that([1, 2, 2, 1, 3, 3]).contains_all([4, 420]);
}
