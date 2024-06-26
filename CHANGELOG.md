# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
