use smoothy::{assert_that, EqualityAssertion, IteratorAssertion};

mod first {
    use super::*;

    #[test]
    fn succeeds() {
        assert_that(vec!["Hello World!"])
            .first()
            .equals("Hello World!");
    }

    #[test]
    #[should_panic = "assertion failed: `(Iterator has first item)`\n           found:  None"]
    fn fails_with_empty_vec() {
        assert_that::<Vec<&str>>(vec![])
            .first()
            .equals("Hello World!");
    }
}

mod second {
    use super::*;

    #[test]
    fn succeeds() {
        assert_that(vec!["First", "Second"])
            .second()
            .equals("Second");
    }

    #[test]
    #[should_panic = "assertion failed: `(Iterator has second item)`\n           found:  None"]
    fn fails_with_empty_vec() {
        assert_that::<Vec<&str>>(vec![])
            .second()
            .equals("Hello World!");
    }
}

mod third {
    use super::*;

    #[test]
    fn succeeds() {
        assert_that(vec!["First", "Second", "Third"])
            .third()
            .equals("Third");
    }

    #[test]
    #[should_panic = "assertion failed: `(Iterator has third item)`\n           found:  None"]
    fn fails_with_empty_vec() {
        assert_that::<Vec<&str>>(vec![])
            .third()
            .equals("Hello World!");
    }
}

mod nth {
    use super::*;

    #[test]
    fn succeeds() {
        assert_that(vec!["First", "Second", "Third"])
            .nth(2)
            .equals("Third");
    }

    #[test]
    #[should_panic = "assertion failed: `(Iterator has 2th item)`\n           found:  None"]
    fn fails_with_empty_vec() {
        assert_that::<Vec<&str>>(vec![])
            .nth(2)
            .equals("Hello World!");
    }
}

mod combined {
    use super::*;

    #[test]
    fn succeeds() {
        let vec = vec!["First", "Second", "Third"];
        assert_that(vec.clone()).first().equals("First");
        assert_that(vec.clone()).second().equals("Second");
        assert_that(vec.clone()).third().equals("Third");
    }
}
