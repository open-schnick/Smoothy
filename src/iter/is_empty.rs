use crate::{implementation::assert, BasicAsserter};
use std::fmt::Debug;

impl<Iterable, Item> BasicAsserter<Iterable>
where
    Iterable: IntoIterator<Item = Item>,
    Item: Debug,
{
    /// Asserts that the [Iterable] is empty.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// let vec: Vec<String> = vec![];
    /// assert_that(vec).is_empty();
    /// ```
    ///
    /// # Panics
    /// When the [Iterable] is not empty.
    pub fn is_empty(self) {
        let mut iter = self.value.into_iter();
        let next_element = iter.next();
        assert(next_element.is_none(), "Iterator is empty", next_element)
    }

    /// Asserts that the [Iterable] is not empty.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// let vec: Vec<String> = vec![String::from("Hello World!")];
    /// assert_that(vec).is_not_empty();
    /// ```
    ///
    /// # Panics
    /// When the [Iterable] is empty.
    pub fn is_not_empty(self) {
        let mut iter = self.value.into_iter();
        let next_element = iter.next();
        assert(
            next_element.is_some(),
            "Iterator is not empty",
            next_element,
        )
    }
}
