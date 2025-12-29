//! Write smooth assertions in a fluent and human-readable way.
//!
//! # Quick Start
//!
//! Start asserting by calling [`assert_that`] on a value.
//! Then chain assertions based on the type you are asserting.
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that("Hello World")
//!     .contains("Hello")
//!     .and() // the "and" is purly cosmetic and can be omitted
//!     .contains("World");
//! ```
//!
//! Smoothy prevents you from writing assertions that make no sense at compile time.
//!
//! This also means that the autocompletion will only show meaningful assertions when writing tests.
//!
//! ```compile_fail
//! # use smoothy::prelude::*;
//! assert_that("Hello World").is_true();
//! ```
//!
//! # Available Assertions
//!
//! 1. [Basic value assertions](#basic-value-assertions)
//! 2. [String-likes](#string-likes)
//! 3. [Result](#result)
//! 4. [Option](#option)
//! 5. [Iterables](#iterables)
//! 6. [Filesystem / Path Assertions](#filesystem--path-assertions)
//! 7. [File Handle Assertions](#file-handle-assertions)
//! 8. [Json](#json)
//! 9. [Accessors](#accessors)
//!
//! ## Basic value assertions
//!
//! There are several assertions available for all types:
//!
//! ### Equality
//!
//! There are two ways to assert equality:
//! - [`is`](trait.EqualityAssertion.html#tymethod.is) compares the value with something of the same type.
//! - [`equals`](trait.EqualityAssertion.html#tymethod.equals) compares the value with something that can be converted into the same type. This is done by using the [Into] trait.
//!
//! [All equality assertions](trait.EqualityAssertion.html)
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that(1).equals(1);
//! assert_that(String::from("Hello")).equals("Hello");
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that(1).not_equals(2);
//! assert_that(String::from("Hello")).not_equals("Hello There");
//! ```
//!
//! Same for [`try_into_equals`](trait.EqualityAssertion.html#tymethod.try_into_equals) and [`try_into_not_equals`](trait.EqualityAssertion.html#tymethod.try_into_not_equals) but here the trait [`TryInto`] is used.
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that(1u8).try_into_equals(1i8);
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that(1u8).try_into_not_equals(2i8);
//! ```
//!
//! When one wants to assert a value while assuring the same type without any conversions is used [`is`](trait.EqualityAssertion.html#tymethod.is) can be used.
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that(1).is(1);
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that(1).is_not(2);
//! ```
//!
//! ### Booleans
//!
//! There are convenience methods for asserting booleans:
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that(true).is_true();
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that(false).is_false();
//! ```
//!
//! Or one can assert with equality:
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that(true).is(true);
//! ```
//!
//! ## String-likes
//!
//! String-likes can be asserted by calling [`contains`](trait.StringAssertion.html#tymethod.contains), [`starts_with`](trait.StringAssertion.html#tymethod.starts_with)
//! or by calling [`matches`](trait.StringAssertion.html#tymethod.matches).
//!
//! [All string assertions](trait.StringAssertion.html)
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that("Hello World")
//!     .contains("Hello")
//!     .and()
//!     .contains("World");
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that("Hello World").starts_with("Hello");
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! # use regex::Regex;
//! assert_that("Hello World").matches(&Regex::new(r"\bHello\b").unwrap());
//! ```
//!
//! ## Result
//!
//! Results can be asserted by calling [`is_err`](trait.ResultAssertion.html#tymethod.is_err) or [`is_ok`](trait.ResultAssertion.html#tymethod.is_ok).
//! Furthermore, their actual content can be asserted as well.
//!
//! [All result assertions](trait.ResultAssertion.html)
//!
//! ### Ok
//!
//! Asserts that the result is [Ok]:
//!
//! ```
//! # use smoothy::prelude::*;
//! let result: Result<u8, ()> = Ok(1);
//! assert_that(result).is_ok();
//! ```
//!
//! Asserting the [Ok]-value:
//!
//! ```
//! # use smoothy::prelude::*;
//! let result: Result<u8, ()> = Ok(1);
//! assert_that(result).is_ok().and_value().equals(1);
//! ```
//!
//! ### Err
//!
//! Asserts that the result is [Err]:
//!
//! ```
//! # use smoothy::prelude::*;
//! let result: Result<(), String> = Err(String::from("Oh no!"));
//! assert_that(result).is_err();
//! ```
//!
//! Asserting the [Err]-value:
//!
//! ```
//! # use smoothy::prelude::*;
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
//! # use smoothy::prelude::*;
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
//! Options can be asserted by calling [`is_none`](trait.OptionAssertion.html#tymethod.is_none) or [`is_some`](trait.OptionAssertion.html#tymethod.is_some).
//! Instances of [`Some`] can be further asserted with [`and_value`](struct.SomeAsserter.html#method.and_value).
//!
//! [All option assertions](trait.OptionAssertion.html)
//!
//! ### None
//!
//! ```
//! # use smoothy::prelude::*;
//! let option: Option<()> = None;
//!
//! assert_that(option).is_none();
//! ```
//!
//! ### Some
//!
//! ```
//! # use smoothy::prelude::*;
//! let option: Option<u8> = Some(1);
//! let asserter: Asserter<u8> = assert_that(option).is_some().and_value();
//! // further assertions
//! asserter.equals(1);
//! ```
//!
//! ## Iterables
//!
//! Anything that implements [`IntoIterator`] can be asserted in content and size.
//!
//! [All iterable assertions](trait.IteratorAssertion.html)
//!
//! ```
//! # use smoothy::prelude::*;
//! let vec: Vec<u8> = vec![];
//! assert_that(vec).is_empty();
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! let vec: Vec<u8> = vec![1, 2, 3];
//! assert_that(vec).is_not_empty();
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! let vec: Vec<u8> = vec![1, 2, 3];
//! assert_that(vec).size().is(3);
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that([1, 2, 3]).first().is(1);
//! assert_that([1, 2, 3]).second().is(2);
//! assert_that([1, 2, 3]).third().is(3);
//! assert_that([1, 2, 3]).nth(0).is(1);
//! ```
//!
//! ### Content assertions
//!
//! The content of iterables can be asserted in different ways depending on the invariants one wants to assert
//!
//! - *Ordered* The order of expected items must match the order of the actual items.
//! - *In Sequence* The expected items must be in the same order as the actual items without any items in between.
//! - *Exclusive* Only the expected items should exist in the iterable and all the expected items should be present in the iterable.
//!
//! | Assertion                  | Ordered | In Sequence | Exclusive | Note                |
//! |----------------------------|---------|-------------|-----------|---------------------|
//! | [`contains`](trait.IteratorAssertion.html#tymethod.contains) / [`contains_all`](trait.IteratorAssertion.html#tymethod.contains_all)    | false   | false       | false     |                     |
//! | [`contains_only`](trait.IteratorAssertion.html#tymethod.contains_only)              | false   | false       | true      |                     |
//! | -                          | false   | true        | false     | Does not make sense |
//! | -                          | false   | true        | true      | Does not make sense |
//! | `contains_in_order` (WIP)    | true    | false       | false     |                     |
//! | -                | true    | false       | true      | Could be useful, but is it needed?                    |
//! | `contains_in_sequence` (WIP) | true    | true        | false     |                     |
//! | [`is`](trait.EqualityAssertion.html#tymethod.is) / [`equals`](trait.EqualityAssertion.html#tymethod.equals)                | true    | true        | true      |                     |
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that([1, 2, 3]).is([1, 2, 3]);
//! assert_that([1, 2, 3]).contains(1);
//! assert_that([1, 2, 3]).contains_all([2, 1]);
//! assert_that([1, 2, 3, 2]).contains_only([3, 1, 2, 2]);
//! ```
//!
//! ### Predicate-based assertions
//!
//! Predicate-based assertions can be used to validate the contents of an iterable.
//!
//! ```
//! # use smoothy::prelude::*;
//! // Assert that all elements match a condition
//! let numbers = vec![2, 4, 6, 8];
//! assert_that(numbers).all_match(|x| x % 2 == 0);
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! let numbers = vec![1, 2, 3];
//! assert_that(numbers).any_match(|x| x % 2 == 0);
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! let numbers = vec![1, 3, 5];
//! assert_that(numbers).none_match(|x| x % 2 == 0);
//! ```
//!
//! ## Filesystem / Path Assertions
//!
//! Path assertions work with any type implementing [`AsRef<Path>`](std::path::Path) such as [`str`], [`String`], [`&Path`](std::path::Path), and [`PathBuf`](std::path::PathBuf).
//!
//! [All path assertions](trait.PathAssertion.html)
//!
//! ### Existence checks
//!
//! ```
//! # use smoothy::prelude::*;
//! # use tempfile::NamedTempFile;
//! let temp_file = NamedTempFile::new().unwrap();
//!
//! assert_that(temp_file.path()).exists();
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! assert_that("/path/that/does/not/exist").not_exists();
//! ```
//!
//! ### Type checks
//!
//! Check if a path points to a regular file:
//!
//! ```
//! # use smoothy::prelude::*;
//! # use tempfile::NamedTempFile;
//! let temp_file = NamedTempFile::new().unwrap();
//!
//! assert_that(temp_file.path()).is_file();
//! ```
//!
//! Check if a path points to a directory:
//!
//! ```
//! # use smoothy::prelude::*;
//! # use tempfile::TempDir;
//! let temp_dir = TempDir::new().unwrap();
//!
//! assert_that(temp_dir.path()).is_directory();
//! ```
//!
//! Check if a path is a symlink (without following it):
//!
//! ```no_run
//! # use smoothy::prelude::*;
//! # use std::os::unix::fs::symlink;
//! # use tempfile::TempDir;
//! let temp_dir = TempDir::new().unwrap();
//! let target = temp_dir.path().join("target");
//! let link = temp_dir.path().join("link");
//! std::fs::write(&target, "content").unwrap();
//! symlink(&target, &link).unwrap();
//!
//! assert_that(&link).is_symlink();
//! ```
//!
//! ## File Handle Assertions
//!
//! File handle assertions work with [`File`](std::fs::File) handles and types that implement [`Borrow<File>`](std::borrow::Borrow).
//! These assertions check the metadata of the file handle itself.
//!
//! [All file assertions](trait.FileAssertion.html)
//!
//! Check if a file handle points to a regular file:
//!
//! ```
//! # use smoothy::prelude::*;
//! # use tempfile::tempfile;
//! let file = tempfile().unwrap();
//!
//! assert_that(file).is_file();
//! ```
//!
//! Check if a file handle points to a directory:
//!
//! ```
//! # use smoothy::prelude::*;
//! # use tempfile::TempDir;
//! # use std::fs::File;
//! let temp_dir = TempDir::new().unwrap();
//! let dir = File::open(temp_dir.path()).unwrap();
//!
//! assert_that(dir).is_directory();
//! ```
//!
//! ## JSON
//!
//! JSON values as used by [`serde_json`] can be asserted about JSON types and values.
//!
//! ```
//! # use smoothy::prelude::*;
//! use serde_json::json;
//! let json = json!({"test": 42});
//!
//! assert_that(json)
//!     .is_object()
//!     .and()
//!     .get("test")
//!     .is_number()
//!     .and()
//!     .equals(42);
//! ```
//!
//! ```
//! # use smoothy::prelude::*;
//! use serde_json::json;
//! let json = json!("test");
//!
//! assert_that(json).is_string().and().equals("test");
//! ```
//!
//! ## Accessors
//!
//! Sometimes one wants to assert only one specific value of a struct.
//! To do so one can use the [`extract`](struct.Asserter.html#method.extract) method.
//!
//! ```
//! # use smoothy::prelude::*;
//! struct Struct(pub String);
//!
//! assert_that(Struct("hello".to_string()))
//!     .extract(|s| s.0.clone())
//!     .equals("hello");
//! ```
// Include code samples from the readme as doc-tests
#![cfg_attr(doctest, doc = include_str!("../README.md"))]
// Render feature requirements in docs.rs
#![cfg_attr(docsrs, feature(doc_cfg))]

mod accessors;
mod assertions;
mod implementation;

#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
#[cfg(feature = "json")]
pub use assertions::json::{JsonObjectAssertion, JsonValueAssertion};
pub use assertions::{
    boolean::BooleanAssertion,
    equality::EqualityAssertion,
    file::FileAssertion,
    iter::IteratorAssertion,
    option::{OptionAssertion, SomeAsserter},
    path::PathAssertion,
    result::{ErrAsserter, OkAsserter, ResultAssertion},
    string::StringAssertion,
};

/// The prelude for smoothy. Contains the most important structs, traits and functions but not all
pub mod prelude {
    pub use crate::{
        assert_that, Asserter, BooleanAssertion, EqualityAssertion, FileAssertion,
        IteratorAssertion, OptionAssertion, PathAssertion, ResultAssertion, StringAssertion,
    };
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    #[cfg(feature = "json")]
    pub use crate::{JsonObjectAssertion, JsonValueAssertion};
}

/// Entrypoint for all assertions
///
/// Enables various assertions on variable based on its type
///
/// # Examples
/// ```
/// # use smoothy::prelude::*;
/// #
/// let assertable = String::from("Hello World!");
/// let assertion: Asserter<String> = assert_that(assertable);
/// // add your assertions here
/// ```
#[must_use = "Without assertions this function does nothing"]
pub const fn assert_that<AssertedType>(value: AssertedType) -> Asserter<AssertedType> {
    Asserter { value }
}

/// Main struct with various assertions on `AssertedType`
pub struct Asserter<AssertedType> {
    pub(crate) value: AssertedType,
}

impl<AssertedType> Asserter<AssertedType> {
    /// Connect two assertions on the same value
    ///
    /// This is purely cosmetic and can be omitted to reduce verbosity
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that(1).equals(1).and().is(1);
    /// ```
    #[track_caller]
    #[must_use = "Connecting an assertion without second assertion does nothing"]
    pub const fn and(self) -> Self {
        self
    }
}

/// Helper construct to prevent implementation of the assertion extension traits outside the crate
mod private {
    pub trait Sealed {}

    impl<AssertedType> Sealed for crate::Asserter<AssertedType> {}
}
