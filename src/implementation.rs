//! Implementation details of the actual assertions and the failure output
#![allow(clippy::redundant_pub_crate, clippy::needless_pass_by_value)]

#[track_caller]
pub(crate) fn assert_ref_equals<T>(actual: &T, expected: T)
where
    T: PartialEq + std::fmt::Debug,
{
    assert_equals(actual, &expected);
}

#[track_caller]
pub(crate) fn assert_ref_not_equals<T>(actual: &T, expected: T)
where
    T: PartialEq + std::fmt::Debug,
{
    assert_not_equals(actual, &expected);
}

#[track_caller]
pub(crate) fn assert_equals<T>(actual: T, expected: T)
where
    T: PartialEq + std::fmt::Debug,
{
    pretty_assertions::assert_eq!(actual, expected);
}

#[track_caller]
pub(crate) fn assert_not_equals<T>(actual: T, expected: T)
where
    T: PartialEq + std::fmt::Debug,
{
    pretty_assertions::assert_ne!(actual, expected);
}

#[track_caller]
pub(crate) fn assert_no_actual(assertable: bool, assertion_desc: &str) {
    type DoesNotMatter = ();
    assert_internal::<DoesNotMatter>(assertable, assertion_desc, None);
}

#[track_caller]
pub(crate) fn assert<T>(assertable: bool, assertion_desc: &str, actual_value: T)
where
    T: std::fmt::Debug,
{
    assert_internal(assertable, assertion_desc, Some(actual_value));
}

// TODO: make the assertion outputs nice and always the same
#[track_caller]
fn assert_internal<T>(assertable: bool, assertion_desc: &str, actual_value: Option<T>)
where
    T: std::fmt::Debug,
{
    match actual_value {
        Some(value) => {
            assert!(
                assertable,
                "assertion failed: `({assertion_desc})`\n           found:  {value:?}",
            );
        }
        None => {
            assert!(assertable, "assertion failed: `({assertion_desc})`",);
        }
    };
}
