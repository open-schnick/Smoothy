//! Write smooth assertions in a fluent and human readable way.
//!
//! # Examples
//!
//! All asserted are stared by calling [`assert_that`] on a value.
//! After that various assertions based on the type of the asserted value can be made.
//!
//! ## Basic value assertions
//!
//! There are several assertions available for all types:
//!
//! ### Equality
//!
//! There are two ways to assert equality:
//! - [`is`](struct.AssertionBuilder.html#method.is) compares the value with something of the same type.
//! - [`equals`](struct.AssertionBuilder.html#method.equals) compares the value with something that can be converted into the same type. This is done by using the [Into] trait.
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
//! Same for [`try_into_equals`](struct.AssertionBuilder.html#method.try_into_equals) and [`try_into_not_equals`](struct.AssertionBuilder.html#method.try_into_not_equals) but here the trait [`TryInto`] is used.
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
//! When one wants to asserts a value while assuring the same type without any conversions is used [`is`](struct.AssertionBuilder.html#method.is)] can be used.
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that(1).is(1);
//! ```
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that(1).is_not(2);
//! ```
//!
//! ## Results
//!
//! Results can be asserted by calling [`is_err`](struct.AssertionBuilder.html#method.is_err) or [`is_ok`](struct.AssertionBuilder.html#method.is_ok).
//! Furthermore their actual content can be asserted as well.
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
//! assert_that(result).is_ok().and_value().equals(1);
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
//! When the [`Err`]-value implements [`PartialEq`] one can use [`and_error_equals`](struct.ErrAsserter.html#method.and_error_equals)
//!
//! ```
//! # use smoothy::assert_that;
//! #[derive(Debug, PartialEq)]
//! struct CustomError(String);
//!
//! let result: Result<(), CustomError> = Err(CustomError(String::from("Oh no!")));
//! assert_that(result).is_err().and_error().equals(CustomError(String::from("Oh no!")));
//! ```
//!
//! Alternatively one can assert the error message (given the error implements [Display](std::fmt::Display)):
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
//! assert_that(result)
//!     .is_err()
//!     .and_error()
//!     .to_string()
//!     .equals("Oh no!");
//! ```
//!
//! ## Option
//!
//! Options can be asserted by calling [`is_none`](struct.AssertionBuilder.html#method.is_none) or [`is_some`](struct.AssertionBuilder.html#method.is_some).
//! Instances of [`Some`] can be further asserted with [`and_value`](struct.SomeAsserter.html#method.and_value).
//!
//! ### None
//!
//! ```
//! # use smoothy::assert_that;
//! let option: Option<()> = None;
//!
//! assert_that(option).is_none();
//! ```
//!
//! ### Some
//!
//! ```
//! # use smoothy::{assert_that, BasicAsserter};
//! let option: Option<u8> = Some(1);
//! let asserter: BasicAsserter<u8> = assert_that(option).is_some().and_value();
//! // further assertions
//! asserter.equals(1);
//! ```
//!
//! ## Iteratables
//!
//! Anything that implements [`IntoIterator`] can be asserted in content and size.
//!
//! ```
//! # use smoothy::assert_that;
//! #
//! let vec: Vec<u8> = vec![];
//! assert_that(vec).is_empty();
//! ```
//!
//! ```
//! # use smoothy::assert_that;
//! #
//! let vec: Vec<u8> = vec![1, 2, 3];
//! assert_that(vec).is_not_empty();
//! ```
//!
#![cfg_attr(feature = "__private_readme_test", doc = include_str!("../README.md"))]
//!

// enable more lint groups
#![deny(clippy::pedantic, clippy::nursery)]
// check documentation
#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
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
    clippy::unwrap_used,
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

pub use option::*;
pub use result::*;

mod equality;
mod implementation;
mod iter;
mod option;
mod result;
mod string;

/// Main struct with various assertions on `AssertedType`
pub struct BasicAsserter<AssertedType> {
    pub(crate) value: AssertedType,
}

/// Entrypoint for all assertions
///
/// Enables various assertions on variable based on its type
///
/// # Examples
/// ```
/// # use smoothy::{assert_that, BasicAsserter};
/// #
/// let assertable = String::from("Hello World!");
/// let assertion: BasicAsserter<String> = assert_that(assertable);
/// // add your assertions here
/// ```
#[must_use]
pub const fn assert_that<AssertedType>(value: AssertedType) -> BasicAsserter<AssertedType> {
    BasicAsserter { value }
}
