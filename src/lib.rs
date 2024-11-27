//! Write smooth assertions in a fluent and human-readable way.
//!
//! 1. [Overview](#overview)
//! 2. [Basic value assertions](#basic-value-assertions)
//! 3. [String-likes](#string-likes)
//! 4. [Result](#result)
//! 5. [Option](#option)
//! 6. [Iterables](#iterables)
//! 7. [Accessors](#accessors)
//!
//! # Overview
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
//! - [`is`](struct.BasicAsserter.html#method.is) compares the value with something of the same type.
//! - [`equals`](struct.BasicAsserter.html#method.equals) compares the value with something that can be converted into the same type. This is done by using the [Into] trait.
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
//! Same for [`try_into_equals`](struct.BasicAsserter.html#method.try_into_equals) and [`try_into_not_equals`](struct.BasicAsserter.html#method.try_into_not_equals) but here the trait [`TryInto`] is used.
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
//! When one wants to assert a value while assuring the same type without any conversions is used [`is`](struct.BasicAsserter.html#method.is) can be used.
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
//! ### Booleans
//!
//! There are convenience methods for asserting booleans:
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that(true).is_true();
//! ```
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that(false).is_false();
//! ```
//!
//! Or one can assert with equality:
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that(true).is(true);
//! ```
//!
//! ## String-likes
//!
//! String-likes can be asserted by calling [`contains_string`](struct.BasicAsserter.html#method.contains_string), [`starts_with_string`](struct.BasicAsserter.html#method.contains_string)
//! or by calling [`is_matching`](struct.BasicAsserter.html#method.is_matching).
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that("Hello World")
//!     .contains_string("Hello")
//!     .and()
//!     .contains_string("World");
//! ```
//!
//! ```
//! # use smoothy::assert_that;
//! assert_that("Hello World").starts_with_string("Hello");
//! ```
//!
//! ```
//! # use smoothy::assert_that;
//! # use regex::Regex;
//! assert_that("Hello World").is_matching(&Regex::new(r"\bHello\b").unwrap());
//! ```
//!
//! ## Result
//!
//! Results can be asserted by calling [`is_err`](struct.BasicAsserter.html#method.is_err) or [`is_ok`](struct.BasicAsserter.html#method.is_ok).
//! Furthermore, their actual content can be asserted as well.
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
//! Asserting the [Err]-value:
//!
//! ```
//! # use smoothy::assert_that;
//! #[derive(Debug, PartialEq)]
//! struct CustomError(String);
//!
//! let result: Result<(), CustomError> = Err(CustomError(String::from("Oh no!")));
//! assert_that(result)
//!     .is_err()
//!     .and_error()
//!     .equals(CustomError(String::from("Oh no!")));
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
//! Options can be asserted by calling [`is_none`](struct.BasicAsserter.html#method.is_none) or [`is_some`](struct.BasicAsserter.html#method.is_some).
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
//! ## Iterables
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
//! ```
//! # use smoothy::assert_that;
//! #
//! let vec: Vec<u8> = vec![1, 2, 3];
//! assert_that(vec).size().is(3);
//! ```
//!
//! ```
//! # use smoothy::assert_that;
//! #
//! assert_that([1, 2, 3]).first().is(1);
//! assert_that([1, 2, 3]).second().is(2);
//! assert_that([1, 2, 3]).third().is(3);
//! assert_that([1, 2, 3]).nth(0).is(1);
//! ```
//!
//! ## Accessors
//!
//! Sometimes one wants to assert only one specific value of a struct.
//! To do so one can use the [`map`](struct.BasicAsserter.html#method.map) method.
//!
//! ```
//! # use smoothy::assert_that;
//! struct Struct(pub String);
//!
//! assert_that(Struct("hello".to_string()))
//!     .map(|s| s.0)
//!     .equals("hello");
//! ```
#![cfg_attr(doctest, doc = include_str!("../README.md"))]
//!

pub use connector::AssertionConnector;
pub use option::SomeAsserter;
pub use result::{ErrAsserter, OkAsserter};

mod accessors;
mod boolean;
mod connector;
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
#[must_use = "Without assertions this function does nothing"]
pub const fn assert_that<AssertedType>(value: AssertedType) -> BasicAsserter<AssertedType> {
    BasicAsserter { value }
}
