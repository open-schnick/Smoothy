use crate::{implementation, BasicAsserter};
use std::fmt::Debug;

impl<SomeValue> BasicAsserter<Option<SomeValue>>
where
    SomeValue: Debug,
{
    /// Asserts that the [Option] is [None].
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let option: Option<String> = None;
    ///
    /// assert_that(option).is_none();
    /// ```
    ///
    /// # Panics
    /// When the [Option] is [Some]
    #[track_caller]
    pub fn is_none(self) {
        implementation::assert(self.value.is_none(), "Option is None", &self.value);
    }
}
