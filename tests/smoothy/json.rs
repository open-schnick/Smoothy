use serde_json::{json, Value};
use smoothy::{
    assert_that, BooleanAssertion, EqualityAssertion, IteratorAssertion, JsonValueAssertion,
};

#[cfg(test)]
mod json_value {
    use super::*;

    #[test]
    fn is_null_cant_be_chained() {
        let _: () = assert_that(json!(null)).is_null();
    }

    #[test]
    #[should_panic = "assertion failed"]
    fn is_null_fails_with_values_other_than_null() {
        assert_that(json!("Hello World!")).is_null();
    }

    #[test]
    fn is_boolean_can_chained() {
        assert_that(json!(true)).is_boolean().and().is_true();
    }

    #[test]
    #[should_panic = "assertion failed"]
    fn is_boolean_fails_with_values_other_than_boolean() {
        assert_that(json!("Hello World!")).is_boolean();
    }

    #[test]
    fn is_number_can_chained() {
        assert_that(json!(42)).is_number().and().equals(42);
    }

    #[test]
    #[should_panic = "assertion failed"]
    fn is_number_fails_with_values_other_than_numbers() {
        assert_that(json!(null)).is_number();
    }

    #[test]
    fn is_string_can_chained() {
        assert_that(json!("Hello World!"))
            .is_string()
            .and()
            .equals("Hello World!");
    }

    #[test]
    #[should_panic = "assertion failed"]
    fn is_string_fails_with_values_other_than_strings() {
        assert_that(json!(42)).is_string();
    }

    #[test]
    fn is_array_can_chained() {
        assert_that(json!([1, 2, 3])).is_array().and().contains(2);
    }

    #[test]
    #[should_panic = "assertion failed"]
    fn is_array_fails_with_values_other_than_arrays() {
        assert_that(json!(42)).is_array();
    }

    #[test]
    fn is_object_can_chained() {
        assert_that(json!({"key": false}))
            .is_object()
            .and()
            .contains(("key".to_string(), Value::Bool(false)));
    }

    #[test]
    #[should_panic = "assertion failed"]
    fn is_object_fails_with_values_other_than_objects() {
        assert_that(json!(42)).is_object();
    }
}

#[cfg(test)]
mod json_object {
    use super::*;
    use smoothy::JsonObjectAssertion;

    #[test]
    fn using_macro() {
        let object = json!({"key": "value"});

        assert_that(object)
            .is_object()
            .and()
            .get("key")
            .is_string()
            .and()
            .equals("value");
    }

    #[test]
    fn using_serde_serialize() {
        #[derive(serde::Serialize)]
        struct Test {
            key: String,
        }

        let value = serde_json::to_value(Test {
            key: "Hello".to_string(),
        })
        .unwrap();

        assert_that(value)
            .is_object()
            .and()
            .get("key")
            .is_string()
            .and()
            .equals("Hello");
    }
}
