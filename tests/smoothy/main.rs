#![allow(missing_docs)]

mod accessors;
mod boolean;
mod equality;
mod file;
mod iter;
mod json;
mod option;
mod path;
mod result;
mod string;

#[macro_export]
#[allow(clippy::missing_panics_doc, clippy::unwrap_used, clippy::shadow_reuse)]
macro_rules! failing_assertion {
    ($assertion:tt) => {{
        let caught_panic = std::panic::catch_unwind(|| $assertion);

        // Capture the panic message
        let caught_panic = caught_panic.err().expect("Assertion should panic");
        let assertion_failed_output = caught_panic.downcast_ref::<String>().unwrap();

        insta::assert_snapshot!(assertion_failed_output);
    }};
    ($assertion:tt, $should_contain:literal) => {
        let caught_panic = std::panic::catch_unwind(|| $assertion);

        // Capture the panic message
        let caught_panic = caught_panic.err().expect("Assertion should panic");
        let assertion_failed_output = caught_panic.downcast_ref::<String>().unwrap();

        assert!(
            assertion_failed_output.contains($should_contain),
            "Actual output:\n{assertion_failed_output}"
        );
    };
}
