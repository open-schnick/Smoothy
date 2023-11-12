# Smoothy

Write smooth assertions in a fluent and human readable way.

## Examples

All asserted are stared by calling [assert_that] on a value.
After that various assertions based on the type of the asserted value can be made.

### Basic value assertions

There are several assertions available for all types

#### Equality

[`AssertionBuilder::<T>.equals()`] and [`not_equals`] take anything that can be converted to the asserted type to improve ergonimics.  
This is done by implementing the [Into] trait

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

Same for [try_equals] and [try_not_equals] but here the trait [TryInto] is used

```rust
# use smoothy::assert_that;
assert_that(1u8).try_equals(1i8);
```

```rust
# use smoothy::assert_that;
assert_that(1u8).try_not_equals(2i8);
```

### Results

Results can be asserted by calling `is_err` or `is_ok`. Futhermore the actual result or the error can be asserted

## TODO:

- `is` to assert equality in a type safe way (compared to the Into<T> stuff)
- vec support (length, contains)
- string support (length, contains, starts_with, ends_with)
- Documentation and testing :D (especially the differnence between `is` and `equals`)
