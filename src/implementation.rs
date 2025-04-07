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
    pretty_assertions::assert_ne!(actual, expected);
}

#[track_caller]
#[allow(clippy::shadow_reuse)]
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
#[allow(clippy::shadow_reuse)]
pub fn assert_no_expected<Actual: Debug>(assertable: bool, actual: Actual, assertion_desc: &str) {
    assert_internal::<Actual, (), ()>(assertable, actual, assertion_desc, None, None);
}

#[track_caller]
#[allow(clippy::shadow_reuse)]
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

// #[allow(clippy::shadow_reuse)]
// fn format_with_expected_content<T: Debug, E: Debug>(
//     actual: T,
//     assertion_desc: &str,
//     expected: E,
// ) -> String {
//     let actual = format!("{actual:?}").green();
//     let expected = format!("{expected:?}").red();
//     format!(
//         "{}\n\nExpecting {assertion_desc}\n{expected}\nbut found:\n{actual}",
//         "Assertion failed!".red()
//     )
// }

// #[test]
// #[allow(clippy::panic)]
// fn asdf() {
// AssertionFormatter::new(true, [1, 2, 3])
//     .with_desc("")
//     .evaluate();
// panic!("{}", format("iterator to be empty", vec![1, 2, 3]));

// assert_that(false).is_true();
//
// assert_no_expected(false, [1, 2, 3], "to be empty");
// assert2(false, "Hello World", "to contain", "Shy Hulud");
// assert2(false, [1, 2, 3], "to contain", 3);
// pretty_assertions::assert_eq!(vec![1, 2, 3], vec![3]);

// let a = AssertionFormatter::new(false, false);
// let b = AssertionFormatter::new(true, false);
// }
// #[derive(Debug, PartialEq)]
// struct AssertionFormatter<A: Debug> {
//     assertion: bool,
//     actual: A,
//     desc: Option<&'static str>,
// }
//
// impl<A: Debug> AssertionFormatter<A> {
//     pub const fn new(assertion: bool, actual: A) -> Self {
//         Self {
//             assertion,
//             actual,
//             desc: None,
//         }
//     }
//
//     pub fn with_desc(self, desc: &'static str) -> Self {
//         Self {
//             desc: Some(desc),
//             ..self
//         }
//     }
//
//     fn format(&self) -> String {
//         String::default()
//     }
//
//     pub fn evaluate(self) {
//         assert!(self.assertion, "{}", self.format());
//     }
// }
