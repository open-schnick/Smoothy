# Smoothy

Write smooth assertions in a fluent and readable way.

[![check](https://github.com/open-schnick/Smoothy/actions/workflows/check.yml/badge.svg)](https://github.com/open-schnick/Smoothy/actions/workflows/check.yml)
[![test](https://github.com/open-schnick/Smoothy/actions/workflows/test.yml/badge.svg)](https://github.com/open-schnick/Smoothy/actions/workflows/test.yml)
[![License](https://img.shields.io/crates/l/smoothy)](https://github.com/open-schnick/Smoothy/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/smoothy)](https://crates.io/crates/smoothy)

## Features

The crate is heavily inspired by [AssertJ](https://assertj.github.io/doc/)

- simple and readable syntax
- assertions based on the type of the asserted value
- assertion values use type conversion traits to make assertions easier to work with

## Why Smoothy?

Standard Rust assertions are quite minimalistic and lack meaningful error messages. Which makes debugging failing tests really hard.

Take the following assertion:

```rust, should_panic
let result: Result<(), String> = Err("Some Error".to_string());

assert!(result.is_ok());
```

This will result in a panic that looks like this:

```shell
thread 'test' panicked at src/lib.rs:42:5:
assertion failed: result.is_ok()
```

well... what is the error here?

Without knowing this, one has to rerun the test with logging or a debugger to find out why the assertion failed.

The same assertion looks like this in Smoothy:

```rust, should_panic
use smoothy::prelude::*;

let result: Result<(), String> = Err("Some Error".to_string());

assert_that(result).is_ok();
```

which leads to the following output:

```shell
thread 'test' panicked at src/lib.rs:42:5:
Assertion failed!

Expected
  Err("Some Error")
to be Ok
```

The output of the default assertions is even worse when you want to assert iterators:

```rust, should_panic
let vec: Vec<i32> = vec![1, 2, 3];

assert!(vec.contains(&4));
```

```shell
thread 'test' panicked at src/lib.rs:42:5:
assertion failed: vec.contains(&4)
```

with Smoothy:

```rust, should_panic
use smoothy::prelude::*;

let vec: Vec<i32> = vec![1, 2, 3];

assert_that(vec).contains(4);
```

```shell
thread 'test' panicked at src/lib.rs:42:5:
Assertion failed!

Expected
  [1, 2, 3]
to contain
  4
```

## How does it work

Start asserting by calling `assert_that` on a value.
Then chain assertions based on the type you are asserting.

```rust
use smoothy::prelude::*;

assert_that("Hello World!").starts_with("Hello").and().contains("World!");
```

Smoothy uses traits to implement different assertions depending on the type you are asserting. This way you can only write assertions that make sense

```rust, compile_fail
use smoothy::prelude::*;

// this obviously makes no sense
assert_that(42).is_true();
```

Smoothy provides a lot of different assertions.

You can find them in the [docs](https://docs.rs/smoothy/latest/smoothy/) or in the structure diagram below.

If you are missing some assertions feel free to open an issue or a pull request :)
