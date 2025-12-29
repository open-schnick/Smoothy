use crate::{implementation, private, Asserter};
use std::fmt::Debug;

/// Specifies various assertions on [`IntoIterator`]. Implemented on [`Asserter`]
///
/// This trait is sealed and cannot be implemented outside Smoothy.
pub trait IteratorAssertion<Iterable, Item>: private::Sealed
where
    Iterable: IntoIterator<Item = Item>,
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
    fn size(self) -> Asserter<usize>;

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
    fn is_not_empty(self) -> Asserter<Vec<Item>>
    where
        Item: Debug;

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
    fn is_empty(self)
    where
        Item: Debug;

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
    fn first(self) -> Asserter<Item>
    where
        Item: Debug;

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
    fn second(self) -> Asserter<Item>
    where
        Item: Debug;

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
    fn third(self) -> Asserter<Item>
    where
        Item: Debug;

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
    fn nth(self, nth: usize) -> Asserter<Item>
    where
        Item: Debug;

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
    fn contains(self, expected: impl Into<Item>) -> Asserter<Vec<Item>>
    where
        Item: Debug + PartialEq;

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
    ) -> Asserter<Vec<Item>>
    where
        Item: Debug + PartialEq;

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
    ) -> Asserter<Vec<Item>>
    where
        Item: Debug + PartialEq;

    /// Asserts that all elements in the iterable match the given predicate.
    ///
    /// Succeeds when the iterator is empty. Use [`is_not_empty`](IteratorAssertion::is_not_empty) first to assert that the iterator is empty.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec![2, 4, 6, 8];
    /// assert_that(vec).all_match(|x| x % 2 == 0);
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec![2, 3, 4];
    /// assert_that(vec).all_match(|x| x % 2 == 0);
    /// ```
    ///
    /// # Panics
    /// When at least one element does not match the predicate.
    #[track_caller]
    fn all_match(self, predicate: impl Fn(&Item) -> bool) -> Asserter<Vec<Item>>
    where
        Item: Debug;

    /// Asserts that at least one element in the iterable matches the given predicate.
    ///
    /// Panics when the iterator is empty.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec![1, 2, 3, 4];
    /// assert_that(vec).any_match(|x| x % 2 == 0);
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec![1, 3, 5];
    /// assert_that(vec).any_match(|x| x % 2 == 0);
    /// ```
    ///
    /// # Panics
    /// When no elements match the predicate.
    #[track_caller]
    fn any_match(self, predicate: impl Fn(&Item) -> bool) -> Asserter<Vec<Item>>
    where
        Item: Debug;

    /// Asserts that no elements in the iterable match the given predicate.
    ///
    /// Returns true when the iterator is empty. Use [`is_not_empty`](IteratorAssertion::is_not_empty) first to assert that the iterator is empty.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec![1, 3, 5];
    /// assert_that(vec).none_match(|x| x % 2 == 0);
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// let vec = vec![1, 2, 3];
    /// assert_that(vec).none_match(|x| x % 2 == 0);
    /// ```
    ///
    /// # Panics
    /// When at least one element matches the predicate.
    #[track_caller]
    fn none_match(self, predicate: impl Fn(&Item) -> bool) -> Asserter<Vec<Item>>
    where
        Item: Debug;
}

impl<Iterable, Item> IteratorAssertion<Iterable, Item> for Asserter<Iterable>
where
    Iterable: IntoIterator<Item = Item>,
{
    fn size(self) -> Asserter<usize> {
        let size = self.value.into_iter().count();
        Asserter { value: size }
    }

    fn is_not_empty(self) -> Asserter<Vec<Item>>
    where
        Item: Debug,
    {
        let actual = self.value.into_iter().collect::<Vec<Item>>();

        implementation::assert_no_expected(
            !actual.is_empty(),
            &actual,
            "to contain at least one item",
        );

        Asserter { value: actual }
    }

    fn is_empty(self)
    where
        Item: Debug,
    {
        let actual = self.value.into_iter().collect::<Vec<Item>>();
        implementation::assert_no_expected(actual.is_empty(), &actual, "to be empty");
    }

    fn first(self) -> Asserter<Item>
    where
        Item: Debug,
    {
        let mut actual = self.value.into_iter();
        let maybe_item = actual.nth(0);

        implementation::assert_no_expected(
            maybe_item.is_some(),
            actual.collect::<Vec<Item>>(),
            "to contain a first item",
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        Asserter { value: item }
    }

    fn second(self) -> Asserter<Item>
    where
        Item: Debug,
    {
        let mut actual = self.value.into_iter();
        let maybe_item = actual.nth(1);

        implementation::assert_no_expected(
            maybe_item.is_some(),
            actual.collect::<Vec<Item>>(),
            "to contain a second item",
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        Asserter { value: item }
    }

    fn third(self) -> Asserter<Item>
    where
        Item: Debug,
    {
        let mut actual = self.value.into_iter();
        let maybe_item = actual.nth(2);

        implementation::assert_no_expected(
            maybe_item.is_some(),
            actual.collect::<Vec<Item>>(),
            "to contain a third item",
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        Asserter { value: item }
    }

    fn nth(self, nth: usize) -> Asserter<Item>
    where
        Item: Debug,
    {
        let mut actual = self.value.into_iter();
        let maybe_item = actual.nth(nth);

        implementation::assert_no_expected(
            maybe_item.is_some(),
            actual.collect::<Vec<Item>>(),
            &format!("to contain a {nth}th item"),
        );

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        Asserter { value: item }
    }

    fn contains(self, expected: impl Into<Item>) -> Asserter<Vec<Item>>
    where
        Item: Debug + PartialEq,
    {
        let actual = self.value.into_iter().collect::<Vec<Item>>();
        let expected_item = expected.into();

        implementation::assert(
            actual.contains(&expected_item),
            &actual,
            "to contain",
            expected_item,
        );

        Asserter { value: actual }
    }

    fn contains_all(
        self,
        expected: impl IntoIterator<Item = impl Into<Item>>,
    ) -> Asserter<Vec<Item>>
    where
        Item: Debug + PartialEq,
    {
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

        Asserter { value: actual }
    }

    fn contains_only(
        self,
        expected: impl IntoIterator<Item = impl Into<Item>>,
    ) -> Asserter<Vec<Item>>
    where
        Item: Debug + PartialEq,
    {
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

        Asserter {
            value: actual_items,
        }
    }

    fn all_match(self, predicate: impl Fn(&Item) -> bool) -> Asserter<Vec<Item>>
    where
        Item: Debug,
    {
        let actual = self.value.into_iter().collect::<Vec<Item>>();

        let non_matching = actual
            .iter()
            .filter(|item| !predicate(item))
            .collect::<Vec<&Item>>();

        implementation::assert_with_additional_info_no_expected(
            non_matching.is_empty(),
            &actual,
            "to have only element matching the predicate",
            "but found elements that did not match",
            &non_matching,
        );

        Asserter { value: actual }
    }

    fn any_match(self, predicate: impl Fn(&Item) -> bool) -> Asserter<Vec<Item>>
    where
        Item: Debug,
    {
        let actual = self.value.into_iter().collect::<Vec<Item>>();

        let has_match = actual.iter().any(predicate);

        implementation::assert_no_expected(
            has_match,
            &actual,
            "to have at least one element matching the predicate",
        );

        Asserter { value: actual }
    }

    fn none_match(self, predicate: impl Fn(&Item) -> bool) -> Asserter<Vec<Item>>
    where
        Item: Debug,
    {
        let actual = self.value.into_iter().collect::<Vec<Item>>();

        let matching = actual
            .iter()
            .filter(|item| predicate(item))
            .collect::<Vec<&Item>>();

        implementation::assert_with_additional_info_no_expected(
            matching.is_empty(),
            &actual,
            "to have no elements matching the predicate",
            "but found elements that matched",
            &matching,
        );

        Asserter { value: actual }
    }
}
