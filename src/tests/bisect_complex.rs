use super::super::*;

#[test]
fn get_new() {
    let values = [1u32, 2, 3, 4, 6, 7, 8, 9];
    let bisect = Bisector::new(&values);

    assert_eq!(values.len(), bisect.values().len());
}

fn run_minor_greater_than_50(version: &semver::Version) -> ConvergeTo<u64, u64> {
    if version.minor >= 50 {
        ConvergeTo::Right(version.minor)
    } else {
        ConvergeTo::Left(version.minor)
    }
}

#[test]
fn bisect_minor_version_is_at_least_50() {
    #[derive(Debug, Eq, PartialEq)]
    pub enum TestableConvergeTo {
        Left(u64),
        Right(u64),
    }

    let versions = [
        semver::Version::new(1, 100, 0),
        semver::Version::new(1, 51, 0),
        semver::Version::new(1, 50, 0),
        semver::Version::new(1, 10, 0),
        semver::Version::new(1, 9, 0),
        semver::Version::new(1, 8, 0),
        semver::Version::new(0, 0, 0),
    ];

    let bisect = Bisector::new(&versions);
    let mut convergence = vec![];

    let mut i = Indices::from_bisector(&bisect);
    while let Step {
        indices,
        result: Some(t),
    } = bisect.bisect(|v| run_minor_greater_than_50(v), i)
    {
        i = indices;

        convergence.push(match t {
            ConvergeTo::Left(l) => TestableConvergeTo::Left(l),
            ConvergeTo::Right(r) => TestableConvergeTo::Right(r),
        });
    }

    assert_eq!(convergence.len(), 3);

    assert_eq!(convergence[0], TestableConvergeTo::Left(10));
    assert_eq!(convergence[1], TestableConvergeTo::Right(51));
    assert_eq!(convergence[2], TestableConvergeTo::Right(50));
}
