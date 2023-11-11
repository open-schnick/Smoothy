use smoothy::assert_that;

#[test]
fn checks_that_the_result_is_ok() {
    let result: Result<String, ()> = Ok(String::new());

    assert_that(result).is_ok();
}

#[test]
fn checks_the_result_and_the_resulting_value() {
    let result: Result<String, String> = Ok(String::from("Hello There"));

    assert_that(result)
        .is_ok()
        .and_value_equals(String::from("Hello There"));
}

#[test]
fn checks_the_result_and_the_resulting_value_with_from_trait() {
    let result: Result<String, String> = Ok(String::from("Hello There"));

    assert_that(result.clone())
        .is_ok()
        .and_value_equals(&String::from("Hello There"));
    assert_that(result).is_ok().and_value_equals("Hello There");
}
