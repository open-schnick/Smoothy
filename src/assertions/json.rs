use crate::{implementation, private, Asserter};
use serde_json::{Map, Number, Value};

/// Specifies various assertions on [`Value`]. Implemented on [`Asserter`]
///
/// This trait is sealed and cannot be implemented outside Smoothy.
pub trait JsonValueAssertion: private::Sealed {
    /// Asserts that the [Value] is a [`Value::Null`].
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use smoothy::prelude::*;
    /// #
    /// let result = json!(null);
    ///
    /// assert_that(result).is_null();
    /// ```
    ///
    /// # Panics
    /// When the [Value] is no [`Value::Null`]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_null(self);

    /// Asserts that the [Value] is a [`Value::Bool`].
    ///
    /// Allows the usage of chained assertions on a bool-value (see [`Asserter`]).
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use smoothy::prelude::*;
    /// #
    /// let result = json!(true);
    ///
    /// assert_that(result).is_boolean().and().is_true();
    /// ```
    ///
    /// # Panics
    /// When the [Value] is no [`Value::Bool`]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_boolean(self) -> Asserter<bool>;

    /// Asserts that the [Value] is a [`Value::Number`].
    ///
    /// Allows the usage of chained assertions on a number-value (see [`Asserter`]).
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use smoothy::prelude::*;
    /// #
    /// let result = json!(42);
    ///
    /// assert_that(result).is_number().and().equals(42);
    /// ```
    ///
    /// # Panics
    /// When the [Value] is no [`Value::Number`]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_number(self) -> Asserter<Number>;

    /// Asserts that the [Value] is a [`Value::String`].
    ///
    /// Allows the usage of chained assertions on a string-value (see [`Asserter`]).
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use smoothy::prelude::*;
    /// #
    /// let result = json!("test");
    ///
    /// assert_that(result).is_string().and().equals("test");
    /// ```
    ///
    /// # Panics
    /// When the [Value] is no [`Value::String`]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_string(self) -> Asserter<String>;

    /// Asserts that the [Value] is a [`Value::Array`].
    ///
    /// Allows the usage of chained assertions on an array-value (see [`Asserter`]).
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use smoothy::prelude::*;
    /// #
    /// let result = json!([null, 42, "test"]);
    ///
    /// assert_that(result).is_array().and().contains(42);
    /// ```
    ///
    /// # Panics
    /// When the [Value] is no [`Value::Array`]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_array(self) -> Asserter<Vec<Value>>;

    /// Asserts that the [Value] is a [`Value::Object`].
    ///
    /// Allows the usage of chained assertions on an object-value (see [`Asserter`]).
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use smoothy::prelude::*;
    /// #
    /// let result = json!({
    ///     "test": 42,
    ///     "other": null,
    /// });
    ///
    /// assert_that(result).is_object().and().get("test").is_number().and().equals(42);
    /// ```
    ///
    /// # Panics
    /// When the [Value] is no [`Value::Object`]
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_object(self) -> Asserter<Map<String, Value>>;
}

impl JsonValueAssertion for Asserter<Value> {
    fn is_null(self) {
        implementation::assert_no_expected(self.value.is_null(), self.value, "JSON to be null");
    }

    fn is_boolean(self) -> Asserter<bool> {
        implementation::assert_no_expected(
            self.value.is_boolean(),
            &self.value,
            "JSON to be a boolean value",
        );

        #[allow(clippy::unreachable)]
        let Value::Bool(value) = self.value
        else {
            unreachable!()
        };

        Asserter { value }
    }

    fn is_number(self) -> Asserter<Number> {
        implementation::assert_no_expected(
            self.value.is_number(),
            &self.value,
            "JSON to be a number",
        );

        #[allow(clippy::unreachable)]
        let Value::Number(value) = self.value
        else {
            unreachable!()
        };

        Asserter { value }
    }

    fn is_string(self) -> Asserter<String> {
        implementation::assert_no_expected(
            self.value.is_string(),
            &self.value,
            "JSON to be a string",
        );

        #[allow(clippy::unreachable)]
        let Value::String(value) = self.value
        else {
            unreachable!()
        };

        Asserter { value }
    }

    fn is_array(self) -> Asserter<Vec<Value>> {
        implementation::assert_no_expected(
            self.value.is_array(),
            &self.value,
            "JSON to be an array",
        );

        #[allow(clippy::unreachable)]
        let Value::Array(value) = self.value
        else {
            unreachable!()
        };

        Asserter { value }
    }

    fn is_object(self) -> Asserter<Map<String, Value>> {
        implementation::assert_no_expected(
            self.value.is_object(),
            &self.value,
            "JSON to be an object",
        );

        #[allow(clippy::unreachable)]
        let Value::Object(value) = self.value
        else {
            unreachable!()
        };

        Asserter { value }
    }
}

/// Specifies various assertions on [`Map<String, Value>`]. Implemented on [`Asserter`]
///
/// This trait is sealed and cannot be implemented outside Smoothy.
pub trait JsonObjectAssertion: private::Sealed {
    /// Convenience function for accessing elements of the JSON Object.
    ///
    /// # Examples
    /// ```
    /// # use serde_json::{json, Map};
    /// # use smoothy::prelude::*;
    /// #
    /// let mut result = Map::new();
    /// result.insert("test".to_string(), json!(42));
    /// result.insert("other".to_string(), json!(null));
    ///
    /// assert_that(result).get("test").equals(json!(42));
    /// ```
    ///
    /// # Panics
    /// When the [Map<String, Value>] does not contain the key.
    fn get(self, key: &str) -> Asserter<Value>;
}

impl JsonObjectAssertion for Asserter<Map<String, Value>> {
    fn get(mut self, key: &str) -> Asserter<Value> {
        let maybe_item = self.value.remove(key);

        implementation::assert(maybe_item.is_some(), &self.value, "to have the key", key);

        #[allow(clippy::unwrap_used)]
        let item = maybe_item.unwrap();

        Asserter { value: item }
    }
}
