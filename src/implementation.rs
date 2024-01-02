//! Implementation details of the actual assertions and the failure output

pub(crate) fn assert_ref_equals<T>(actual: &T, expected: T)
where
    T: PartialEq + std::fmt::Debug,
{
    assert_equals(actual, &expected);
}

pub(crate) fn assert_ref_not_equals<T>(actual: &T, expected: T)
where
    T: PartialEq + std::fmt::Debug,
{
    assert_not_equals(actual, &expected);
}

pub(crate) fn assert_equals<T>(actual: T, expected: T)
where
    T: PartialEq + std::fmt::Debug,
{
    pretty_assertions::assert_eq!(actual, expected);
}

pub(crate) fn assert_not_equals<T>(actual: T, expected: T)
where
    T: PartialEq + std::fmt::Debug,
{
    pretty_assertions::assert_ne!(actual, expected);
}

// TODO: make the assertion outputs nice and always the same
pub(crate) fn assert<T>(assertable: bool, assertion_desc: &str, actual_value: T)
where
    T: std::fmt::Debug,
{
    assert!(
        assertable,
        "assertion failed: `({assertion_desc})`\n           found:  {actual_value:?}"
    );
}
