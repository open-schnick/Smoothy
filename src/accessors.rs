use crate::{assert_that, Asserter};

impl<AssertedType> Asserter<AssertedType> {
    /// Extracts the value of an asserted value
    ///
    /// This allows for a property access of inner values or functions
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// struct Struct(pub String);
    ///
    /// assert_that(Struct("hello".to_string()))
    ///     .extract(|s| s.0)
    ///     .equals("hello");
    /// ```
    ///
    /// The extraction could be hidden by a trait
    ///
    /// ```rust
    /// # use smoothy::prelude::*;
    /// struct Struct(pub String);
    ///
    /// pub trait SmoothyExt {
    ///     fn inner_value(self) -> Asserter<String>;
    /// }
    ///
    /// impl SmoothyExt for Asserter<Struct> {
    ///     fn inner_value(self) -> Asserter<String> {
    ///         self.extract(|s| s.0)
    ///     }
    /// }
    ///
    /// assert_that(Struct("hello".to_string()))
    ///     .inner_value()
    ///     .equals("hello");
    /// ```
    #[track_caller]
    #[must_use = "Extracting a value without assertion does nothing"]
    pub fn extract<NewAssertedType>(
        self,
        extractor: impl FnOnce(AssertedType) -> NewAssertedType,
    ) -> Asserter<NewAssertedType> {
        let extracted = extractor(self.value);
        assert_that(extracted)
    }
}

impl<AssertedType> Asserter<AssertedType>
where
    AssertedType: ToString,
{
    /// Converts the assertable to a string for further assertions
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// let asserter: Asserter<String> = assert_that(42).to_string();
    /// // further assertions
    /// asserter.equals("42");
    /// ```
    #[track_caller]
    #[must_use = "Transforming the asserted value does not assert anything"]
    pub fn to_string(self) -> Asserter<String> {
        Asserter {
            value: self.value.to_string(),
        }
    }
}
