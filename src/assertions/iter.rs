use crate::{implementation, private, AssertionConnector, BasicAsserter};
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

    /// Asserts that the iterable contains the item at least once in any place in the iterator
    ///
    /// [See top-level docs for more details on content assertions](index.html#content-assertions)
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec!["First", "Second", "Third"];
    ///
    /// assert_that(vec).contains("Second");
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec!["First", "Second", "Third"];
    ///
    /// assert_that(vec).contains("Does not exist");
    /// ```
    ///
    /// # Panics
    /// When the Iterator does not contain the expected item.
    #[track_caller]
    fn contains(self, expected: impl Into<Item>) -> AssertionConnector<Vec<Item>>;

    /// Asserts that the iterable contains each item at least once in any place in the iterator
    ///
    /// [See top-level docs for more details on content assertions](index.html#content-assertions)
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec!["First", "Second", "Third"];
    ///
    /// assert_that(vec).contains_all(["Second", "First"]);
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec!["First", "Second", "Third"];
    ///
    /// assert_that(vec).contains_all(["Does not exist", "Also does not exist"]);
    /// ```
    ///
    /// # Panics
    /// When the Iterator does not contain at least one of the expected items.
    #[track_caller]
    fn contains_all(
        self,
        expected_items: impl IntoIterator<Item = impl Into<Item>>,
    ) -> AssertionConnector<Vec<Item>>;

    /// Asserts that the iterable contains only the expected items any place in the iterator
    ///
    /// [See top-level docs for more details on content assertions](index.html#content-assertions)
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec!["A", "B", "A", "C"];
    ///
    /// // Order does not matter
    /// assert_that(vec).contains_only(["C", "A", "A", "B"]);
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec!["A", "B", "C"];
    ///
    /// // Missing "C"
    /// assert_that(vec).contains_only(["A", "B"]);
    /// ```
    ///
    /// # Panics
    /// When the Iterator contains additional elements other than the specified ones.
    #[track_caller]
    fn contains_only(
        self,
        expected_items: impl IntoIterator<Item = impl Into<Item>>,
    ) -> AssertionConnector<Vec<Item>>;
}

impl<Iterable, Item> IteratorAssertion<Iterable, Item> for BasicAsserter<Iterable>
where
    Iterable: IntoIterator<Item = Item>,
    Item: Debug + PartialEq,
{
    fn size(self) -> BasicAsserter<usize> {
        let size = self.value.into_iter().count();
        BasicAsserter { value: size }
    }

    fn is_not_empty(self) -> AssertionConnector<Vec<Item>> {
        let actual = self.value.into_iter().collect::<Vec<Item>>();

        implementation::assert_no_expected(
            !actual.is_empty(),
            &actual,
            "to contain at least one item",
        );

        AssertionConnector { value: actual }
    }

    fn is_empty(self) {
        let actual = self.value.into_iter().collect::<Vec<Item>>();
        implementation::assert_no_expected(actual.is_empty(), &actual, "to be empty");
    }

    fn first(self) -> BasicAsserter<Item> {
        let mut actual = self.value.into_iter();
        let maybe_item = actual.nth(0);

        implementation::assert_no_expected(
            maybe_item.is_some(),
            actual.collect::<Vec<Item>>(),
            "to contain a first item",
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        BasicAsserter { value: item }
    }

    fn second(self) -> BasicAsserter<Item> {
        let mut actual = self.value.into_iter();
        let maybe_item = actual.nth(1);

        implementation::assert_no_expected(
            maybe_item.is_some(),
            actual.collect::<Vec<Item>>(),
            "to contain a second item",
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        BasicAsserter { value: item }
    }

    fn third(self) -> BasicAsserter<Item> {
        let mut actual = self.value.into_iter();
        let maybe_item = actual.nth(2);

        implementation::assert_no_expected(
            maybe_item.is_some(),
            actual.collect::<Vec<Item>>(),
            "to contain a third item",
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        BasicAsserter { value: item }
    }

    fn nth(self, nth: usize) -> BasicAsserter<Item> {
        let mut actual = self.value.into_iter();
        let maybe_item = actual.nth(nth);

        implementation::assert_no_expected(
            maybe_item.is_some(),
            actual.collect::<Vec<Item>>(),
            &format!("to contain a {nth}th item"),
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        BasicAsserter { value: item }
    }

    fn contains(self, expected: impl Into<Item>) -> AssertionConnector<Vec<Item>> {
        let actual = self.value.into_iter().collect::<Vec<Item>>();
        let expected_item = expected.into();

        implementation::assert(
            actual.contains(&expected_item),
            &actual,
            "to contain",
            expected_item,
        );

        AssertionConnector { value: actual }
    }

    fn contains_all(
        self,
        expected: impl IntoIterator<Item = impl Into<Item>>,
    ) -> AssertionConnector<Vec<Item>> {
        let actual = self.value.into_iter().collect::<Vec<Item>>();
        #[allow(clippy::shadow_reuse)]
        let expected = expected.into_iter().map(Into::into).collect::<Vec<Item>>();

        let not_found = expected
            .iter()
            .filter(|ele| !actual.contains(ele))
            .collect::<Vec<&Item>>();

        implementation::assert_with_additional_info(
            not_found.is_empty(),
            &actual,
            "to contain all of",
            &expected,
            "but did not contain",
            &not_found,
        );

        AssertionConnector { value: actual }
    }

    fn contains_only(
        self,
        expected: impl IntoIterator<Item = impl Into<Item>>,
    ) -> AssertionConnector<Vec<Item>> {
        let actual_items = self.value.into_iter().collect::<Vec<Item>>();
        let expected_items = expected.into_iter().map(Into::into).collect::<Vec<Item>>();

        let mut expected_item_indices = (0..expected_items.len()).collect::<Vec<_>>();

        let mut extra_items_in_actual = Vec::with_capacity(actual_items.len());

        for actual in &actual_items {
            #[allow(clippy::unwrap_used)]
            let matching_available_item_found_in_expected =
                expected_item_indices
                    .iter()
                    .position(|available_expected_item| {
                        expected_items.get(*available_expected_item).unwrap() == actual
                    });
            match matching_available_item_found_in_expected {
                None => {
                    // Element not found in expected -> actual has more elements than expected
                    extra_items_in_actual.push(actual);
                }
                Some(index) => {
                    // Actual was matched by an item in expected -> removing the index from the available items
                    expected_item_indices.remove(index);
                }
            }
        }

        implementation::assert_with_additional_info(
            extra_items_in_actual.is_empty(),
            &actual_items,
            "to contain only",
            &expected_items,
            "but found extra items",
            extra_items_in_actual,
        );

        implementation::assert_with_additional_info(
            expected_item_indices.is_empty(),
            &actual_items,
            "to contain only",
            &expected_items,
            "but did not contain",
            #[allow(clippy::unwrap_used)]
            expected_item_indices
                .iter()
                .map(|expected_item_index| expected_items.get(*expected_item_index).unwrap())
                .collect::<Vec<&Item>>(),
        );

        AssertionConnector {
            value: actual_items,
        }
    }
}
