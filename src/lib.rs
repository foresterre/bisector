//! # Bisector
//!
//! Bisector is a flexible and stateless implementation of the [`bisection method`](https://en.wikipedia.org/wiki/Bisection_method)
//! for Rust.
//!
//! For a detailed description of how the bisection method can be applied, see [`Bisector::bisect`].
//! The [`Bisector`] also provides the [`Bisector::try_bisect`] method which may be used when
//! the convergence function is fallible.
//!
//! # Examples
//!
//! Examples can be found in the examples folder of this crate ([Examples on GitHub](https://github.com/foresterre/bisector/tree/main/examples)).
//! You can also look at the tests, which can be found in the `src/tests/` folder ([Tests on Github](https://github.com/foresterre/bisector/tree/main/src/tests)).
//!
//! An [example](https://github.com/foresterre/cargo-msrv/blob/6c18525f4c1dcb888b6e4392cef52c8ecdf1adc6/src/search_methods/bisect.rs)
//! of actual usage can be found in the `cargo msrv` project.
//!
//! NB: Linked revision of usage in `cargo msrv` was implemented before [`Bisector::try_bisect`] was added.
//! To cover a fallible case in the convergence function, you may want to use [`Bisector::try_bisect`]
//! over [`Bisector::bisect`].
//!
//! [`Bisector`]: crate::Bisector
//! [`Bisector::bisect`]: crate::Bisector::bisect
//! [`Bisector::try_bisect`]: crate::Bisector::try_bisect

#[cfg(test)]
mod tests;

pub(crate) mod error;

use std::fmt::Debug;

/// Error returned by [`Indices::try_from_bisector`], when the slice given to [`Bisector::new`]
/// is empty.
///
/// [`Indices::try_from_bisector`]: crate::Indices::try_from_bisector
/// [`Bisector::new`]: crate::Bisector::new
pub use error::EmptySliceError;

/// Stateless implementation of the bisection method.
#[derive(Debug)]
pub struct Bisector<'v, T> {
    view: &'v [T],
}

impl<'v, T> Bisector<'v, T> {
    /// Create a new [`Bisector`] for a given view.
    pub fn new(view: &'v [T]) -> Self {
        Self { view }
    }

    /// A view of the slice as known to the bisector.
    ///
    /// NB: This is always the complete view, regardless of any bisection steps which might have
    /// taken place.
    pub fn view(&self) -> &'v [T] {
        self.view
    }

    /// Stateless implementation of the bisection method.
    ///
    /// This method takes a convergence function `f` and indices `indices`.
    ///
    /// The convergence function `f` is used to determine towards which side of the current view, the
    /// bisection should progress.
    ///
    /// Since this implementation is stateless, no internal state about the progression is stored.
    /// Instead, indices of point from which the bisection method should be applied must be provided.
    /// The method returns a [`Step`] struct as output. This `indices` property of this struct contains
    /// the indices which point to the view to which the convergence function `f` converged, during
    /// this step. In other words, the `Step::indices` may be used as input for the `indices` parameter
    /// for the next bisection iteration.
    ///
    /// As also described above, the `indices` argument must be the left and right index which point
    /// to the view used by the current step of the bisection. The left and right index must be valid
    /// indices for the slice held by the [`Bisector`] (also called the `view`).
    ///
    /// See also:
    /// * [`Bisector::try_bisect`]: A variant of [`bisect`] which can be used when the convergence function
    ///   is fallible.
    ///
    /// [`Step`]: crate::Step
    /// [`Bisector`]: crate::Bisector
    /// [`Bisector::try_bisect`]: crate::Bisector::try_bisect
    /// [`bisect`]: crate::Bisector::bisect
    // Specialized version of a binary search which allows to return a special output value produced by the search
    // function, instead of a customary index to the being searched value. Also, unlike the customary
    // `while indices.left != indices.right` loop, the method takes the current indices (left, right)
    // and returns the resulting indices as part of it's output.
    pub fn bisect<F, L, R>(&self, f: F, indices: Indices) -> Step<L, R>
    where
        F: FnOnce(&T) -> ConvergeTo<L, R>,
    {
        let Indices { left, right } = indices;

        if left == right {
            return Step {
                indices,
                result: None,
            };
        }

        let middle = indices.middle();

        match f(&self.view[middle]) {
            ConvergeTo::Left(out) => Step {
                indices: Indices {
                    left,
                    right: middle,
                },
                result: Some(ConvergeTo::Left(out)),
            },
            ConvergeTo::Right(out) => Step {
                indices: Indices {
                    left: middle + 1,
                    right,
                },
                result: Some(ConvergeTo::Right(out)),
            },
        }
    }

    /// This method can be used when the convergence function is fallible.
    /// Otherwise exactly the same as [`Bisector::bisect`].
    ///
    /// When the bisection is already in a converged state for the given indices,
    /// the method returns with an Ok Result.
    ///
    /// [`Bisector::bisect`]: crate::Bisector::bisect
    pub fn try_bisect<F, E, L, R>(&self, f: F, indices: Indices) -> Result<Step<L, R>, E>
    where
        F: FnOnce(&T) -> Result<ConvergeTo<L, R>, E>,
    {
        let Indices { left, right } = indices;

        if left == right {
            return Ok(Step {
                indices,
                result: None,
            });
        }

        let middle = indices.middle();

        match f(&self.view[middle])? {
            ConvergeTo::Left(out) => Ok(Step {
                indices: Indices {
                    left,
                    right: middle,
                },
                result: Some(ConvergeTo::Left(out)),
            }),
            ConvergeTo::Right(out) => Ok(Step {
                indices: Indices {
                    left: middle + 1,
                    right,
                },
                result: Some(ConvergeTo::Right(out)),
            }),
        }
    }
}

/// The left and right indices, which in combination with the slice held by the [`Bisector`], provide
/// the view on which a bisection step can be applied.
///
/// The [`Bisector::bisect`] and [`Bisector::try_bisect`] methods take these `Indices` as input, and
/// produce a new `Indices` copy as output (the indices of the one step further converged area are
/// produced as output).
///
/// [`Bisector::bisect`]: crate::Bisector::bisect
/// [`Bisector::try_bisect`]: crate::Bisector::try_bisect
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Indices {
    pub left: usize,
    pub right: usize,
}

impl Indices {
    /// Create a new pair of indices.
    ///
    /// **When used with Bisector**
    ///
    /// The left index must be smaller or equal to the right index `left <= right`,
    /// for the [`Bisector`] to work properly.
    /// It's up to the implementor to uphold this requirement.
    ///
    /// [`Bisector`]: crate::Bisector
    pub fn new(left_index: usize, right_index: usize) -> Self {
        Self {
            left: left_index,
            right: right_index,
        }
    }

    /// Re-use the slice of the [`Bisector`] to determine the starting indices.
    /// The returned indices will be the complete range of the slice, i.e. from index `0` to
    /// index `|slice| - 1` (length of slice minus 1, i.e. the last index of the slice).
    ///
    /// NB: The slice given to [`Bisector`] **must not be empty**.
    ///
    /// ### Undefined behaviour
    ///
    /// If the slice given to [`Bisector`] is empty, the resulting behaviour may not be as expected.
    /// In addition, semantically different behaviour may occur when compiling with `rustc`
    /// debug or release mode.
    ///
    /// **Debug mode**
    ///
    /// In rustc debug mode, if the slice is empty, i.e. the length of the slice is `0`, this function
    /// will panic, by virtue of debug mode out of bounds checking.
    ///
    /// **Release mode**
    ///
    /// In rustc release mode, if the slice is empty, i.e. the length of the slice is `0`, the value
    /// set to the `right` index will underflow, resulting in undefined behaviour.
    ///
    /// [`Bisector`]: crate::Bisector
    pub fn from_bisector<T>(bisector: &Bisector<T>) -> Self {
        Self {
            left: 0,
            right: bisector.view.len() - 1,
        }
    }

    /// Re-use the slice of the [`Bisector`] to determine the starting indices.
    ///
    /// The returned indices will be the complete range of the slice, i.e. from index `0` to
    /// index `|slice| - 1` (length of slice minus 1, i.e. the last index of the slice).
    ///
    /// The slice given to [`Bisector`] must not be empty. If it is, an [`EmptySliceError`]
    /// `Err` result will be returned..
    ///
    /// [`Bisector`]: crate::Bisector
    /// [`EmptySliceError`]: crate::EmptySliceError
    pub fn try_from_bisector<T>(bisector: &Bisector<T>) -> Result<Self, EmptySliceError> {
        if !bisector.view.is_empty() {
            Ok(Self {
                left: 0,
                right: bisector.view.len() - 1,
            })
        } else {
            Err(EmptySliceError)
        }
    }

    /// Computes the mid-point between the left and right indices.
    /// Uses integer division, so use with care.
    #[inline]
    pub fn middle(&self) -> usize {
        (self.left + self.right) / 2
    }
}

/// The output of one bisection step.
///
/// The output consists of two parts:
/// * `indices`: The indices of the converged view
/// * `result`: The output of the convergence function
///
/// The `indices` can be used to progress the bisection. The outputted indices are the indices of
/// the next step.
///
/// The `result` is `Option::Some` when the convergence method can progress further,
/// and `Option::None` when, for the given input indices, the bisector was already in a converged state.
/// This property can be used to stop the bisection when used in a loop; i.e., you can loop
/// `while let Some(result) = step.result`.
///
/// The `result`, when `Option::Some`, also contains the output of the convergence as
/// `ConvergeTo::Left(left)` or `ConvergeTo::Right(right)`, where `left` and `right` are the left and
/// right convergence outputs respectively.  
pub struct Step<L, R> {
    pub indices: Indices,
    pub result: Option<ConvergeTo<L, R>>,
}

/// The direction towards which a [`step`] of the [`Bisector`] should converge.
///
/// [`Bisector`]: crate::Bisector
/// [`step`]: crate::Bisector::bisect
pub enum ConvergeTo<Left, Right> {
    Left(Left),
    Right(Right),
}

impl<Left, Right> ConvergeTo<Left, Right> {
    pub fn try_into_left(self) -> Option<Left> {
        if let Self::Left(left) = self {
            Some(left)
        } else {
            None
        }
    }

    pub fn try_into_right(self) -> Option<Right> {
        if let Self::Right(right) = self {
            Some(right)
        } else {
            None
        }
    }
}
