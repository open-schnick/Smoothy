use crate::BasicAsserter;
use core::fmt::Debug;

impl<Iterable, Item> BasicAsserter<Iterable>
where
    Iterable: IntoIterator<Item = Item>,
    Item: Debug,
{
    /// Convenience function for getting the size of the Iterator.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::assert_that;
    /// #
    /// let vec: Vec<String> = vec!["Hello World!".to_string()];
    /// assert_that(vec).size().is(1);
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::assert_that;
    /// #
    /// let vec: Vec<String> = vec![];
    /// assert_that(vec).size().is(42);
    /// ```
    ///
    /// # Panics
    /// When the Iterator does not contain a first element.
    #[track_caller]
    #[must_use = "Accessing the count of the iterable does not assert anything"]
    pub fn size(self) -> BasicAsserter<usize> {
        let size = self.value.into_iter().count();
        BasicAsserter { value: size }
    }
}
