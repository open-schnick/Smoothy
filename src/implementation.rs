//! Implementation details of the actual assertions and the failure output
#![allow(
    clippy::redundant_pub_crate,
    clippy::needless_pass_by_value,
    clippy::panic
)]

use colored::Colorize;
use std::fmt::Debug;

#[track_caller]
pub(crate) fn assert_ref_equals<T>(actual: &T, expected: T)
where
    T: PartialEq + Debug,
{
    assert_equals(actual, &expected);
}

#[track_caller]
pub(crate) fn assert_ref_not_equals<T>(actual: &T, expected: T)
where
    T: PartialEq + Debug,
{
    assert_not_equals(actual, &expected);
}

#[track_caller]
pub(crate) fn assert_equals<T>(actual: T, expected: T)
where
    T: PartialEq + Debug,
{
    pretty_assertions::assert_eq!(actual, expected);
}

#[track_caller]
pub(crate) fn assert_not_equals<T>(actual: T, expected: T)
where
    T: PartialEq + Debug,
{
    assert(actual != expected, actual, "to not match", expected);
}

#[track_caller]
pub fn assert<Actual: Debug, Expected: Debug>(
    assertable: bool,
    actual: Actual,
    assertion_desc: &str,
    expected: Expected,
) {
    assert_internal::<Actual, Expected, ()>(
        assertable,
        actual,
        assertion_desc,
        Some(expected),
        None,
    );
}

#[track_caller]
pub fn assert_no_expected<Actual: Debug>(assertable: bool, actual: Actual, assertion_desc: &str) {
    assert_internal::<Actual, (), ()>(assertable, actual, assertion_desc, None, None);
}

#[track_caller]
pub fn assert_with_additional_info<Actual: Debug, Expected: Debug, Additional: Debug>(
    assertable: bool,
    actual: Actual,
    assertion_desc: &str,
    expected: Expected,
    additional_desc: &str,
    additional: Additional,
) {
    assert_internal(
        assertable,
        actual,
        assertion_desc,
        Some(expected),
        Some((additional_desc, additional)),
    );
}

#[track_caller]
#[allow(clippy::shadow_reuse)]
fn assert_internal<Actual: Debug, Expected: Debug, Additional: Debug>(
    assertable: bool,
    actual: Actual,
    assertion_desc: &str,
    expected: Option<Expected>,
    additional: Option<(&str, Additional)>,
) {
    if assertable {
        return;
    }

    let actual = format!("{actual:?}").red();

    let mut message = format!(
        "{}\n\nExpected\n  {actual}\n{assertion_desc}",
        "Assertion failed!".red()
    );

    if let Some(expected) = expected {
        let mut expected = format!("{expected:?}");
        // potential additional info is more important than expected
        if additional.is_none() {
            expected = expected.green().to_string();
        }
        message = format!("{message}\n  {expected}");
    }

    if let Some((additional_desc, additional)) = additional {
        let additional = format!("{additional:?}").green();
        message = format!("{message}\n{additional_desc}\n  {additional}");
    }

    panic!("{message}")
}
