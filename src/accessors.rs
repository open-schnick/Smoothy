use crate::{assert_that, BasicAsserter};

impl<AssertedType> BasicAsserter<AssertedType> {
    /// Maps the inner value to an assertion of a different type
    ///
    /// This allows for a property access of inner values or functions
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// struct Struct(pub String);
    ///
    /// assert_that(Struct("hello".to_string()))
    ///     .map(|s| s.0)
    ///     .equals("hello");
    /// ```
    ///
    /// The mapping could be hidden by a trait
    ///
    /// ```rust
    /// # use smoothy::prelude::*;
    /// struct Struct(pub String);
    ///
    /// pub trait SmoothyExt {
    ///     fn inner_value(self) -> BasicAsserter<String>;
    /// }
    ///
    /// impl SmoothyExt for BasicAsserter<Struct> {
    ///     fn inner_value(self) -> BasicAsserter<String> {
    ///         self.map(|s| s.0)
    ///     }
    /// }
    ///
    /// assert_that(Struct("hello".to_string()))
    ///     .inner_value()
    ///     .equals("hello");
    /// ```
    #[track_caller]
    #[must_use = "Mapping an assertion without second assertion does nothing"]
    pub fn map<NewAssertedType>(
        self,
        mapping: impl FnOnce(AssertedType) -> NewAssertedType,
    ) -> BasicAsserter<NewAssertedType> {
        let mapped = mapping(self.value);
        assert_that(mapped)
    }
}
