# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.5](https://github.com/open-schnick/Smoothy/compare/v0.8.4...v0.8.5) - 2025-11-02

### Added

- add any_matches, all_matches and none_matches iterator assertions

### Other

- fix outdated ref in README.md

## [0.8.4](https://github.com/open-schnick/Smoothy/compare/v0.8.3...v0.8.4) - 2025-10-25

### Added

- add file and path assertions to check existence and type

### Other

- update the README
- remove deprecated clippy lint
- use updated release-plz action

## [0.8.3](https://github.com/open-schnick/Smoothy/compare/v0.8.2...v0.8.3) - 2025-04-12

### Added

- deprecate .map() in favour of .extract()
- use similar_asserts instead of pretty_assertions as this allows us to specify labels for the inputs
- use own assertions impl instead of assert_ne macro
- improve output of failing assertions by printing actual and expected and using colored outputs

### Other

- inline unused public method
- use snapshots testing for failing equality asserts
- force colored outputs for ci runs
- remove deprecated cargo command
- fix output of contains_only and add missing test
- use insta for snapshot testing assertion messages
- use .contains on iterator instead of using .any
- add usage tests for json
- move json section, add link to top-level docs for iterator assertions

## [0.8.2](https://github.com/open-schnick/Smoothy/compare/v0.8.1...v0.8.2) - 2025-04-06

### Added

- Add support for JSON values

## [0.8.1](https://github.com/open-schnick/Smoothy/compare/v0.8.0...v0.8.1) - 2025-03-07

### Added

- move 'matches' string assertion behind feature flag which is enabled per default

### Other

- fix top level rustdoc links

## [0.8.0](https://github.com/open-schnick/Smoothy/compare/v0.7.0...v0.8.0) - 2025-03-07

### Added

- add "contains_all" on IteratorAssertions
- add "contains_only" on IteratorAssertions
- add "contains" on IteratorAssertions
- [**breaking**] add PartialEq trait bound for IteratorAssertion
- [**breaking**] seal all assertion traits to prevent accidental breaking changes downstream
- is_not_empty returns assertion connector for further iterator assertions

### Other

- add iterable assertions doc
- add missing clippy flags to check step
- reorder declarations in lib.rs
- update deps
- add implementation for assertions without actual value
- remove redundant test
- update README.md to include renames

## [0.7.0](https://github.com/open-schnick/Smoothy/compare/v0.6.0...v0.7.0) - 2024-12-15

### Other

- adapt test folder structure
- [**breaking**] use a trait-based approach for defining boolean assertions
- [**breaking**] rename string assertions to make them more idiomatic
- group assertions into own module
- copy implementation trait bounds into assertion trait definitions

## [0.6.0](https://github.com/open-schnick/Smoothy/compare/v0.5.2...v0.6.0) - 2024-12-14

### Added

- introduce prelude to ease future complexity of imports

### Other

- [**breaking**] rename contains_string to contains in StringAssertion
- [**breaking**] use a trait-based approach for defining iterator assertions
- [**breaking**] use a trait-based approach for defining result assertions
- [**breaking**] use a trait-based approach for defining string assertions
- [**breaking**] use a trait-based approach for defining option assertions
- [**breaking**] use a trait-based approach for defining equality assertions
- move to_string into accessors
- remove redundant examples from README.md
- move lint into lib.rs

## [0.5.2](https://github.com/open-schnick/Smoothy/compare/v0.5.1...v0.5.2) - 2024-11-27

### Added

- add starts_with_string for asserting string-likes

### Other

- fix typos and dead links in docs

## [0.5.1](https://github.com/open-schnick/Smoothy/compare/v0.5.0...v0.5.1) - 2024-08-31

### Added
- add is_true and is_false convenience functions for booleans

## [0.5.0](https://github.com/open-schnick/Smoothy/compare/v0.4.5...v0.5.0) - 2024-08-29

### Other
- move lint definition out of lib.rs and into Cargo.toml
- check for nightly formatting and switch clippy action
- switch to nightly formatting
- remove redundant lockfile generation for tests
- fix whitespace in code comments of BasicConnector
- [**breaking**] rename contains on string-likes to contains_string
- add 'map' to structural overview and fix typo
- add toc

## [0.4.5](https://github.com/open-schnick/Smoothy/compare/v0.4.4...v0.4.5) - 2024-08-25

### Added
- make all public functions track the caller

## [0.4.4](https://github.com/open-schnick/Smoothy/compare/v0.4.3...v0.4.4) - 2024-05-15

### Added
- add BasicAsserter.map as basic accessor

### Other
- *(tests)* fix clippy warning
- fix typo in must_use

## [0.4.3](https://github.com/open-schnick/Smoothy/compare/v0.4.2...v0.4.3) - 2024-03-03

### Added
- introduce is_matching for string-likes

### Other
- small improvements to pipeline
- disable running tests on beta
- change contains error message

## [0.4.2](https://github.com/open-schnick/Smoothy/compare/v0.4.1...v0.4.2) - 2024-03-01

### Added
- add contains for string-likes

### Other
- *(tests)* add missing allow to remove warning
- add test for combined usage of iter indices
- add missing size docs

## [0.4.1](https://github.com/open-schnick/Smoothy/compare/v0.4.0...v0.4.1) - 2024-01-09

### Added
- add size function for accessing the size of a iterable

## [0.4.0](https://github.com/open-schnick/Smoothy/compare/v0.3.3...v0.4.0) - 2024-01-03

### Added
- introduce iter indices for easy access to the values of an iterable

### Other
- [**breaking**] remove private feature flag by replacing readme inclusion with doctest attribute
- add missing indices tests

## [0.3.3](https://github.com/open-schnick/Smoothy/compare/v0.3.2...v0.3.3) - 2024-01-02

### Added
- basic equality assertions now return AssertionConnector to enable multiple assertions on the same type
- introduce AssertionConnector

### Fixed
- clippy warnings in tests

### Other
- add missing ; to README sample code
- migrate structure diagram to mermaid and update README
- add must_use with explanation to functions that transform the assertable but do not assert

## [0.3.2](https://github.com/open-schnick/Smoothy/compare/v0.3.1...v0.3.2) - 2023-12-11

### Added
- add is_empty and is_not_empty for iterables

### Fixed
- clippy warnings

### Other
- move is_not_empty into own module

## [0.3.1](https://github.com/open-schnick/Smoothy/compare/v0.3.0...v0.3.1) - 2023-12-04

### Fixed
- now `is_ok` doesn't need PartialEq

### Other
- update docs to reflect refactoring
- add missing test for to_string
- move to_string implementation into own string module
- fix action triggers
- reduce manual overhead by running release-plz on every main commit

## [0.3.0](https://github.com/open-schnick/Smoothy/compare/v0.2.0...v0.3.0) - 2023-11-27

### Added
- add option asserting
- introduce to_string conversion on BasicAsserter
- improve assert implementation output

### Other
- introduce automated releases with release-plz
- rename module basic to equality
- add docs
- [**breaking**] ResultAsserters now support recursive assertions and conversions
- [**breaking**] rename AssertionBuilder to BasicAsserter
- fix links in README
