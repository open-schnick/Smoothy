# Smoothy

Write smooth assertions in a fluent and human readable way.

## Examples

All asserted are stared by calling [assert_that] on a value.
After that various assertions based on the type of the asserted value can be made.

### Basic value assertions

There are several assertions available for all types:

#### Equality

[equals](struct.AssertionBuilder.html#method.equals) and [not_equals](struct.AssertionBuilder.html#method.not_equals) take anything that can be converted to the asserted type to improve ergonimics.  
This is done by implementing the [Into] trait.

```rust
# use smoothy::assert_that;
assert_that(1).equals(1);
assert_that(String::from("Hello")).equals("Hello");
```

```rust
# use smoothy::assert_that;
assert_that(1).not_equals(2);
assert_that(String::from("Hello")).not_equals("Hello There");
```

Same for [try_equals](struct.AssertionBuilder.html#method.try_equals) and [try_not_equals](struct.AssertionBuilder.html#method.try_not_equals) but here the trait [TryInto] is used.

```rust
# use smoothy::assert_that;
assert_that(1u8).try_equals(1i8);
```

```rust
# use smoothy::assert_that;
assert_that(1u8).not_try_equals(2i8);
```

### Results

Results can be asserted by calling [is_err](struct.AssertionBuilder.html#method.is_err) or [is_ok](struct.AssertionBuilder.html#method.is_ok).
Futhermore their actual content can be asserted aswell.

#### Ok

Basic assertion that the result is [Ok]:

```rust
# use smoothy::assert_that;
let result: Result<u8, ()> = Ok(1);
assert_that(result).is_ok();
```

Asserting the [Ok]-value:

```rust
# use smoothy::assert_that;
let result: Result<u8, ()> = Ok(1);
assert_that(result).is_ok().and_value_equals(1);
```

#### Err

Basic assertion that the result is [Err]:

```rust
# use smoothy::assert_that;
let result: Result<(), String> = Err(String::from("Oh no!"));
assert_that(result).is_err();
```

When the [Err]-value implements [PartialEq] one can use [and_error_equals](struct.ErrAsserter.html#method.and_error_equals)

```rust
# use smoothy::assert_that;
#[derive(Debug, PartialEq)]
struct CustomError(String);

let result: Result<(), CustomError> = Err(CustomError(String::from("Oh no!")));
assert_that(result).is_err().and_error_equals(CustomError(String::from("Oh no!")));
```

Alternativly one can assert the error message (given the error implements [Display](std::fmt::Display)):

```rust
# use smoothy::assert_that;
# use std::fmt::{Display, Formatter};
#[derive(Debug)]
struct CustomError(String);
#
# impl Display for CustomError {
#     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
#         write!(f, "{}", self.0)
#     }
# }

let result: Result<(), CustomError> = Err(CustomError(String::from("Oh no!")));
assert_that(result).is_err().and_error_as_string_equals("Oh no!");
```

## TODO:

- `is` to assert equality in a type safe way (compared to the `Into<T>` stuff)
- vec support (length, contains)
- string support (length, contains, starts_with, ends_with)
- Documentation and testing :D (especially the differnence between `is` and `equals`)
