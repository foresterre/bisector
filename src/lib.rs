#[cfg(test)]
mod tests;

use std::fmt::Debug;

#[derive(Debug)]
pub struct Bisector<'v, T> {
    values: &'v [T],
}

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
    /// **Panics**
    ///
    /// Panics if the slice is empty, i.e. the length of the slice is `0`.
    ///
    /// [`Bisector`]: crate::Bisector
    pub fn from_bisector<T>(bisector: &Bisector<T>) -> Self {
        Self {
            left: 0,
            right: bisector.values.len() - 1,
        }
    }

    // Computes the mid-point between the left and right indices.
    // Uses integer division, so use with care.
    #[inline]
    pub(crate) fn middle(&self) -> usize {
        (self.left + self.right) / 2
    }
}

pub struct Step<L, R> {
    pub indices: Indices,
    pub result: Option<ConvergeTo<L, R>>,
}

impl<'v, T> Bisector<'v, T> {
    pub fn new(values: &'v [T]) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &'v [T] {
        self.values
    }

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

        match f(&self.values[middle]) {
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
}

pub enum ConvergeTo<Left, Right> {
    Left(Left),
    Right(Right),
}
