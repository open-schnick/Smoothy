#![allow(missing_docs)]

mod accessors;
mod boolean;
mod equality;
mod iter;
mod json;
mod option;
mod result;
mod string;

#[macro_export]
#[allow(clippy::missing_panics_doc, clippy::unwrap_used, clippy::shadow_reuse)]
macro_rules! failing_assertion {
    ($assertion:tt) => {{
        let caught_panic = std::panic::catch_unwind(|| $assertion);

        // Capture the panic message
        let caught_panic = caught_panic.err().unwrap();
        let assertion_failed_output = caught_panic.downcast_ref::<String>().unwrap();

        insta::assert_snapshot!(assertion_failed_output);
    }};
}
