use crate::{implementation, implementation::assert, private, AssertionConnector, BasicAsserter};
use std::fmt::Debug;

/// Specifies various assertions on [`IntoIterator`]. Implemented on [`BasicAsserter`]
///
/// This trait is sealed and cannot be implemented outside Smoothy.
pub trait IteratorAssertion<Iterable, Item>: private::Sealed
where
    Iterable: IntoIterator<Item = Item>,
    Item: Debug + PartialEq,
{
    /// Convenience function for getting the size of the Iterator.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec!["Hello World!".to_string()];
    /// assert_that(vec).size().is(1);
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec![];
    /// assert_that(vec).size().is(42);
    /// ```
    ///
    /// # Panics
    /// When the Iterator does not contain a first element.
    #[track_caller]
    #[must_use = "Accessing the count of the iterable does not assert anything"]
    fn size(self) -> BasicAsserter<usize>;

    /// Asserts that the Iterable is not empty.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec![String::from("Hello World!")];
    /// assert_that(vec).is_not_empty();
    /// ```
    ///
    /// # Panics
    /// When the Iterable is empty.
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_not_empty(self) -> AssertionConnector<Vec<Item>>;

    /// Asserts that the Iterable is empty.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec![];
    /// assert_that(vec).is_empty();
    /// ```
    ///
    /// # Panics
    /// When the Iterable is not empty.
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_empty(self);

    /// Convenience function for accessing the first (0th) element of the Iterable.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec!["Hello World!".to_string()];
    /// assert_that(vec).first().equals("Hello World!");
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec![];
    /// assert_that(vec).first().equals("Hello World!");
    /// ```
    ///
    /// # Panics
    /// When the Iterator does not contain a first element.
    #[track_caller]
    #[must_use = "Accessing the first element only asserts that size > 0. If you want to assert the size use assert_that(iter).size().equals(1) instead"]
    fn first(self) -> BasicAsserter<Item>;

    /// Convenience function for accessing the second (1st) element of the Iterable.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec!["First".to_string(), "Second".to_string()];
    /// assert_that(vec).second().equals("Second");
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec![];
    /// assert_that(vec).second().equals("Hello World!");
    /// ```
    ///
    /// # Panics
    /// When the Iterator does not contain a second element.
    #[track_caller]
    #[must_use = "Accessing the second element only asserts that size > 1. If you want to assert the size use assert_that(iter).size().equals(2) instead"]
    fn second(self) -> BasicAsserter<Item>;

    /// Convenience function for accessing the third (2nd) element of the Iterable.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec![
    ///     "First".to_string(),
    ///     "Second".to_string(),
    ///     "Third".to_string(),
    /// ];
    /// assert_that(vec).third().equals("Third");
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec![];
    /// assert_that(vec).third().equals("Hello World!");
    /// ```
    ///
    /// # Panics
    /// When the Iterator does not contain a third element.
    #[track_caller]
    #[must_use = "Accessing the third element only asserts that size > 2. If you want to assert the size use assert_that(iter).size().equals(3) instead"]
    fn third(self) -> BasicAsserter<Item>;

    /// Convenience function for accessing the nth element of the Iterable.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec![
    ///     "First".to_string(),
    ///     "Second".to_string(),
    ///     "Third".to_string(),
    /// ];
    /// assert_that(vec).nth(2).equals("Third");
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec: Vec<String> = vec![];
    /// assert_that(vec).nth(0).equals("Hello World!");
    /// ```
    ///
    /// # Panics
    /// When the Iterator does not contain a nth element.
    #[track_caller]
    #[must_use = "Accessing the nth element only asserts that size is at least nth. If you want to assert the size use assert_that(iter).size().equals(nth) instead"]
    fn nth(self, nth: usize) -> BasicAsserter<Item>;
}

impl<Iterable, Item> IteratorAssertion<Iterable, Item> for BasicAsserter<Iterable>
where
    Iterable: IntoIterator<Item = Item>,
    Item: Debug,
{
    fn size(self) -> BasicAsserter<usize> {
        let size = self.value.into_iter().count();
        BasicAsserter { value: size }
    }

    fn is_not_empty(self) -> AssertionConnector<Vec<Item>> {
        let vec = self.value.into_iter().collect::<Vec<Item>>();
        implementation::assert(!vec.is_empty(), "Iterator is not empty", &vec);

        AssertionConnector { value: vec }
    }

    fn is_empty(self) {
        let vec = self.value.into_iter().collect::<Vec<Item>>();
        assert(vec.is_empty(), "Iterator is empty", vec);
    }

    fn first(self) -> BasicAsserter<Item> {
        let mut iter = self.value.into_iter();
        let maybe_item = iter.nth(0);

        implementation::assert(maybe_item.is_some(), "Iterator has first item", &maybe_item);

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        BasicAsserter { value: item }
    }

    fn second(self) -> BasicAsserter<Item> {
        let mut iter = self.value.into_iter();
        let maybe_item = iter.nth(1);

        implementation::assert(
            maybe_item.is_some(),
            "Iterator has second item",
            &maybe_item,
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        BasicAsserter { value: item }
    }

    fn third(self) -> BasicAsserter<Item> {
        let mut iter = self.value.into_iter();
        let maybe_item = iter.nth(2);

        implementation::assert(maybe_item.is_some(), "Iterator has third item", &maybe_item);

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        BasicAsserter { value: item }
    }

    fn nth(self, nth: usize) -> BasicAsserter<Item> {
        let mut iter = self.value.into_iter();
        let maybe_item = iter.nth(nth);

        implementation::assert(
            maybe_item.is_some(),
            &format!("Iterator has {nth}th item"),
            &maybe_item,
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        BasicAsserter { value: item }
    }
}
