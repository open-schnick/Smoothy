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
    if !assertable {
        panic!(
            "assertion failed:\n  Expected: {}\n  But was:  {:?}",
            assertion_desc, actual_value
        );
    }
}
