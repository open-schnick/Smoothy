use crate::{implementation, BasicAsserter};
use core::fmt::Debug;

impl<Iterable, Item> BasicAsserter<Iterable>
where
    Iterable: IntoIterator<Item = Item>,
    Item: Debug,
{
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
    pub fn first(self) -> BasicAsserter<Item> {
        let mut iter = self.value.into_iter();
        let maybe_item = iter.nth(0);

        implementation::assert(maybe_item.is_some(), "Iterator has first item", &maybe_item);

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        BasicAsserter { value: item }
    }

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
    pub fn second(self) -> BasicAsserter<Item> {
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
    pub fn third(self) -> BasicAsserter<Item> {
        let mut iter = self.value.into_iter();
        let maybe_item = iter.nth(2);

        implementation::assert(
            maybe_item.is_some(),
            "Iterator has second item",
            &maybe_item,
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        BasicAsserter { value: item }
    }

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
    pub fn nth(self, nth: usize) -> BasicAsserter<Item> {
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
