use std::fmt::Debug;

#[derive(Debug)]
pub struct Bisector<'v, T> {
    values: &'v [T],
}

#[derive(Debug, Copy, Clone)]
pub struct Indices {
    pub left: usize,
    pub right: usize,
}

impl Indices {
    pub fn from_bisector<T>(bisector: &Bisector<T>) -> Self {
        Self {
            left: 0,
            right: bisector.values().len() - 1,
        }
    }

    #[inline]
    pub fn middle(&self) -> usize {
        (self.left + self.right) / 2
    }
}

pub struct Step<L: Debug, R: Debug> {
    pub indices: Indices,
    pub result: Option<Either<L, R>>,
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
    pub fn bisect<F, L: Debug, R: Debug>(&self, f: F, indices: Indices) -> Step<L, R>
    where
        F: FnOnce(&T) -> Either<L, R>,
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
            Either::Left(out) => Step {
                indices: Indices {
                    left,
                    right: middle,
                },
                result: Some(Either::Left(out)),
            },
            Either::Right(out) => Step {
                indices: Indices {
                    left: middle + 1,
                    right,
                },
                result: Some(Either::Right(out)),
            },
        }
    }
}

#[derive(Debug)]
pub enum Either<Left: Debug, Right: Debug> {
    Left(Left),
    Right(Right),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FailOutput {
        message: String,
        value: u32,
    }

    fn f(k: u32) -> Either<u32, FailOutput> {
        if k > 7 {
            Either::Left(k)
        } else {
            Either::Right(FailOutput {
                message: "Failed check".into(),
                value: k,
            })
        }
    }

    #[test]
    fn get_new() {
        let values = [1u32, 2, 3, 4, 6, 7, 8, 9];
        let bisect = Bisector::new(&values);

        assert_eq!(values.len(), bisect.values().len());
    }

    #[test]
    fn bisect() {
        let values = [1u32, 2, 3, 4, 6, 7, 8, 9];
        let bisect = Bisector::new(&values);

        let mut i = Indices::from_bisector(&bisect);
        while let Step {
            indices,
            result: Some(t),
        } = bisect.bisect(|v| f(*v), i)
        {
            println!("out = {:?}", t);
            i = indices;
        }
    }

    #[test]
    fn bisect2() {
        let values = [9u32, 8, 7, 6, 5, 4, 3, 2, 1];
        let bisector = Bisector::new(&values);

        let mut failures = vec![];
        let mut middle = None;

        let mut i = Indices::from_bisector(&bisector);
        while let Step {
            indices,
            result: Some(t),
        } = bisector.bisect(|v| f(*v), i)
        {
            println!("{:?}, out = {:?}", i, t);
            i = indices;
            middle = Some(indices.middle());

            if let Either::Right(f) = t {
                failures.push(f);
            }
        }

        println!("Failures: {:?}", failures);
        println!("Last middle: {:?}", middle);
    }
}

#[cfg(test)]
mod tests_with_commands {
    use super::*;

    #[derive(Debug)]
    struct FailOutput {
        message: String,
        value: u64,
        exit_code: i32,
    }

    fn run_external_command(version: &semver::Version) -> Either<u64, FailOutput> {
        let command = std::process::Command::new("ewc")
            .arg(&format!("{}", version.minor))
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let status = command.wait_with_output().unwrap();

        if status.status.success() {
            Either::Left(version.minor)
        } else {
            Either::Right(FailOutput {
                message: String::from_utf8(status.stderr).unwrap(),
                value: version.minor,
                exit_code: status.status.code().unwrap(),
            })
        }
    }

    fn run_minor_greater_than_50(version: &semver::Version) -> Either<u64, u64> {
        if version.minor >= 50 {
            Either::Right(version.minor)
        } else {
            Either::Left(version.minor)
        }
    }

    #[test]
    fn bisect_with_command() {
        let versions = [
            semver::Version::new(2, 100, 0),
            semver::Version::new(1, 50, 0),
            semver::Version::new(1, 50, 0),
            semver::Version::new(1, 10, 0),
            semver::Version::new(1, 9, 0),
            semver::Version::new(1, 8, 0),
            semver::Version::new(0, 0, 0),
        ];

        let bisect = Bisector::new(&versions);

        let mut middle = None;
        let mut failures = vec![];

        let mut i = Indices::from_bisector(&bisect);
        while let Step {
            indices,
            result: Some(t),
        } = bisect.bisect(|v| run_external_command(v), i)
        {
            println!("{:?}, out = {:?}", i, t);
            i = indices;
            middle = Some(indices.middle());

            if let Either::Right(f) = t {
                failures.push(f);
            }
        }

        println!("Failures: {:?}", failures);
        println!("Last middle: {:?}", middle);
    }
}
