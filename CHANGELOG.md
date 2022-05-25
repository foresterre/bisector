# Changelog

## [Unreleased]

[Unreleased]: https://github.com/foresterre/bisector/compare/v0.4.0...HEAD

## [0.4.0] - 2022-05-25

### Added

* Add fallible function `Indices::try_from_bisector` to create a valid `Indices` instance

### Documentation

* Improved documentation of `Indices::from_bisector`, by better explaining how it may cause problems when calling `bisect`
  or `try_bisect`.
* Suggest usage of  `Indices::try_from_bisector` over `Indices::from_bisector`

[0.4.0]: https://github.com/foresterre/bisector/compare/v0.3.0...v0.4.0

