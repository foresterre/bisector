use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct EmptySliceError;

impl Display for EmptySliceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Expected a non-empty slice, but the given slice was empty (len = 0)"
        ))
    }
}

impl std::error::Error for EmptySliceError {}
