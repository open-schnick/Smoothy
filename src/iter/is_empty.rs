use crate::{implementation::assert, BasicAsserter};
use std::fmt::Debug;

impl<Iterable, Item> BasicAsserter<Iterable>
where
    Iterable: IntoIterator<Item = Item>,
    Item: Debug,
{
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
    pub fn is_empty(self) {
        let mut iter = self.value.into_iter();
        let next_element = iter.next();
        assert(next_element.is_none(), "Iterator is empty", next_element);
    }
}
