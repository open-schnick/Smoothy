use crate::failing_assertion;
use smoothy::{assert_that, IteratorAssertion};

#[test]
fn succeeds() {
    let vec = vec![1, 2, 3, 4, 5];

    assert_that(vec).contains(2);
}

#[test]
fn can_be_chained_with_connectors() {
    let vec = vec![1, 2, 3, 4, 5];

    assert_that(vec)
        .contains(1)
        .and()
        .contains(2)
        .and()
        .contains(3);
}

#[test]
fn expected_element_is_converted_to_asserted_type() {
    let vec: Vec<u16> = vec![1, 2, 3, 4, 5];
    let expected: u8 = 5;

    assert_that(vec).contains(expected);
}

#[test]
fn fails_when_iterator_does_not_contain_element() {
    failing_assertion!({
        let vec = vec!["A", "B", "C"];

        assert_that(vec).contains("Does not exist in iterator");
    });
}
