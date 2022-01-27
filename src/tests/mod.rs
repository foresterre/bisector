use crate::ConvergeTo;

mod bisect;
mod bisect_complex;

#[cfg(feature = "testing_external_program_ewc")]
mod external_program;
mod indices;
mod try_bisect;

pub(in crate::tests) fn input_1_to_10() -> Vec<u32> {
    (1..=10).collect()
}

pub(in crate::tests) fn input_1() -> Vec<u32> {
    vec![1]
}

pub(in crate::tests) fn input_empty() -> Vec<u32> {
    vec![]
}

// A helper trait, since our ConvergeTo<L, R> struct does not implement debug, as we don't want to
// force L: Debug and R: Debug on implementers.
trait UnwrapConvergeTo<L, R> {
    fn unwrap_converge_left(self) -> L;

    fn unwrap_converge_right(self) -> R;
}

impl<L, R> UnwrapConvergeTo<L, R> for ConvergeTo<L, R> {
    fn unwrap_converge_left(self) -> L {
        match self {
            Self::Left(l) => l,
            Self::Right(_) => panic!("Expected left, but got right"),
        }
    }

    fn unwrap_converge_right(self) -> R {
        match self {
            Self::Left(_) => panic!("Expected right, but got left"),
            Self::Right(r) => r,
        }
    }
}
