# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
