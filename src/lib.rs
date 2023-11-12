#![doc = include_str!("../README.md")]
// enable more lint groups
#![deny(clippy::pedantic, clippy::nursery)]
// check documentation
// #![deny(missing_docs, rustdoc::broken_intra_doc_links)]
// enable extra restriction lints
#![deny(
    clippy::as_conversions,
    clippy::as_underscore,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::deref_by_slicing,
    clippy::else_if_without_else,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::try_err,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::unwrap_in_result,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    clippy::mem_forget,
    clippy::missing_assert_message,
    clippy::mod_module_files,
    clippy::mixed_read_write_in_expression,
    clippy::multiple_inherent_impl,
    clippy::needless_raw_strings,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::pub_without_shorthand,
    clippy::ref_patterns,
    clippy::same_name_method,
    clippy::semicolon_outside_block,
    clippy::shadow_reuse,
    clippy::string_add,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::tests_outside_test_module,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::wildcard_enum_match_arm
)]
#![allow(
    clippy::shadow_reuse,
    clippy::missing_panics_doc,
    clippy::redundant_pub_crate,
    clippy::needless_pass_by_value
)]
mod basic;
mod implementation;
mod result;

pub use result::*;

pub struct AssertionBuilder<AssertedType> {
    pub(crate) value: AssertedType,
}

pub const fn assert_that<AssertedType>(value: AssertedType) -> AssertionBuilder<AssertedType> {
    AssertionBuilder { value }
}
