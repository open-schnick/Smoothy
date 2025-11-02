use crate::failing_assertion;
use smoothy::{assert_that, IteratorAssertion};

#[cfg(test)]
mod succeeds {
    use super::*;

    #[test]
    fn when_one_element_matches() {
        let vec = vec![1, 2, 3, 4];
        assert_that(vec).any_match(|x| x % 2 == 0);
    }

    #[test]
    fn when_all_elements_match() {
        let vec = vec![2, 4, 6, 8];
        assert_that(vec).any_match(|x| x % 2 == 0);
    }

    #[test]
    fn can_be_chained_with_connectors() {
        let vec = vec![1, 2, 3, 4];
        assert_that(vec).any_match(|x| x % 2 == 0).and().contains(2);
    }

    #[test]
    fn works_with_complex_predicates() {
        let vec = vec![5, 10, 15, 20];
        assert_that(vec).any_match(|x| x > &15 && x < &25);
    }
}

#[cfg(test)]
mod fails {
    use super::*;

    #[test]
    fn when_no_elements_match() {
        failing_assertion!({
            let vec = vec![1, 3, 5, 7];
            assert_that(vec).any_match(|x| x % 2 == 0);
        });
    }

    #[test]
    fn when_empty() {
        failing_assertion!({
            let vec: Vec<i32> = vec![];
            assert_that(vec).any_match(|x| x % 2 == 0);
        });
    }
}
