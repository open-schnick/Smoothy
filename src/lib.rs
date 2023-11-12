//! Write smooth assertions in a fluent and human readable way.
//!
//! # Examples
//!
//! All asserted are stared by calling [assert_that] on a value.
//! After that various assertions based on the type of the asserted value can be made.
//!
//! ## Basic value assertions
//!
//! There are several assertions available for all types:
//!
//! ### Equality
//!
//! [equals](struct.AssertionBuilder.html#method.equals) and [not_equals](struct.AssertionBuilder.html#method.not_equals) take anything that can be converted to the asserted type to improve ergonimics.  
//! This is done by implementing the [Into] trait.
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that(1).equals(1);
//! assert_that(String::from("Hello")).equals("Hello");
//! ```
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that(1).not_equals(2);
//! assert_that(String::from("Hello")).not_equals("Hello There");
//! ```
//!
//! Same for [try_into_equals](struct.AssertionBuilder.html#method.try_into_equals) and [try_into_not_equals](struct.AssertionBuilder.html#method.try_into_not_equals) but here the trait [TryInto] is used.
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that(1u8).try_into_equals(1i8);
//! ```
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that(1u8).try_into_not_equals(2i8);
//! ```
//!
//! ## Results
//!
//! Results can be asserted by calling [is_err](struct.AssertionBuilder.html#method.is_err) or [is_ok](struct.AssertionBuilder.html#method.is_ok).
//! Futhermore their actual content can be asserted aswell.
//!
//! ### Ok
//!
//! Basic assertion that the result is [Ok]:
//!
//! ```
//! # use smoothy::assert_that;
//! let result: Result<u8, ()> = Ok(1);
//! assert_that(result).is_ok();
//! ```
//!
//! Asserting the [Ok]-value:
//!
//! ```
//! # use smoothy::assert_that;
//! let result: Result<u8, ()> = Ok(1);
//! assert_that(result).is_ok().and_value_equals(1);
//! ```
//!
//! ### Err
//!
//! Basic assertion that the result is [Err]:
//!
//! ```
//! # use smoothy::assert_that;
//! let result: Result<(), String> = Err(String::from("Oh no!"));
//! assert_that(result).is_err();
//! ```
//!
//! When the [Err]-value implements [PartialEq] one can use [and_error_equals](struct.ErrAsserter.html#method.and_error_equals)
//!
//! ```
//! # use smoothy::assert_that;
//! #[derive(Debug, PartialEq)]
//! struct CustomError(String);
//!
//! let result: Result<(), CustomError> = Err(CustomError(String::from("Oh no!")));
//! assert_that(result).is_err().and_error_equals(CustomError(String::from("Oh no!")));
//! ```
//!
//! Alternativly one can assert the error message (given the error implements [Display](std::fmt::Display)):
//!
//! ```
//! # use smoothy::assert_that;
//! # use std::fmt::{Display, Formatter};
//! #[derive(Debug)]
//! struct CustomError(String);
//! #
//! # impl Display for CustomError {
//! #     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//! #         write!(f, "{}", self.0)
//! #     }
//! # }
//!
//! let result: Result<(), CustomError> = Err(CustomError(String::from("Oh no!")));
//! assert_that(result).is_err().and_error_to_string_equals("Oh no!");
//! ```

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
