# Changelog

## [Unreleased]

### Fixed

* The last element of a slice is now also given to the convergence function. Previously
  `Indices::from_bisector` and `Indices::try_from_bisector` returned `|slice| - 1` as right index, so the
  last element could never be reached.

### Changed

* The right index of `Indices` is now the exclusive upper bound of the view, i.e. the element it points
  to is not part of the view. `Indices::from_bisector` and `Indices::try_from_bisector` therefore return
  `|slice|` as right index. When the bisection converges, the right index may now be `|slice|`, which is
  not a valid index for the slice.
* `Indices::from_bisector` no longer underflows on an empty slice; it returns indices which are already
  converged instead.

[Unreleased]: https://github.com/foresterre/bisector/compare/v0.4.0...HEAD

## [0.4.0] - 2022-05-25

### Added

* Add fallible function `Indices::try_from_bisector` to create a valid `Indices` instance

### Documentation

* Improved documentation of `Indices::from_bisector`, by better explaining how it may cause problems when calling `bisect`
  or `try_bisect`.
* Suggest usage of  `Indices::try_from_bisector` over `Indices::from_bisector`

[0.4.0]: https://github.com/foresterre/bisector/compare/v0.3.0...v0.4.0

