use crate::failing_assertion;
use smoothy::{assert_that, IteratorAssertion};

#[cfg(test)]
mod succeeds {
    use super::*;

    #[test]
    fn when_all_elements_match() {
        let vec = vec![2, 4, 6, 8];
        assert_that(vec).all_match(|x| x % 2 == 0);
    }

    #[test]
    fn when_empty() {
        let vec: Vec<i32> = vec![];
        assert_that(vec).all_match(|x| x % 2 == 0);
    }

    #[test]
    fn can_be_chained_with_connectors() {
        let vec = vec![2, 4, 6, 8];
        assert_that(vec).all_match(|x| x % 2 == 0).and().contains(4);
    }

    #[test]
    fn works_with_complex_predicates() {
        let vec = vec![10, 20, 30, 40];
        assert_that(vec).all_match(|x| x > &5 && x < &50);
    }
}

#[cfg(test)]
mod fails {
    use super::*;

    #[test]
    fn when_not_all_match() {
        failing_assertion!({
            let vec = vec![2, 3, 4, 6];
            assert_that(vec).all_match(|x| x % 2 == 0);
        });
    }
}
