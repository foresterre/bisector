use super::super::*;

#[derive(Debug)]
struct FailOutput {
    message: String,
    value: u64,
    exit_code: i32,
}

fn run_external_command(version: &semver::Version) -> ConvergeTo<u64, FailOutput> {
    // Requires https://github.com/foresterre/exit-with-code to be installed and available on the PATH
    let command = std::process::Command::new("ewc")
        .arg(&format!("{}", version.minor))
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let status = command.wait_with_output().unwrap();

    if status.status.success() {
        ConvergeTo::Left(version.minor)
    } else {
        ConvergeTo::Right(FailOutput {
            message: String::from_utf8(status.stderr).unwrap(),
            value: version.minor,
            exit_code: status.status.code().unwrap(),
        })
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
        semver::Version::new(0, 0, 0), // will succeed with ewc 0
    ];

    let bisect = Bisector::new(&versions);

    let mut failures = vec![];

    let mut i = Indices::from_bisector(&bisect);
    while let Step {
        indices,
        result: Some(t),
    } = bisect.bisect(|v| run_external_command(v), i)
    {
        i = indices;

        if let ConvergeTo::Right(f) = t {
            failures.push(f);
        }
    }

    assert_eq!(failures.len(), 2);

    assert_eq!(failures[0].exit_code, 10);
    assert!(failures[0].message.is_empty());
    assert_eq!(failures[0].value, 10);

    assert_eq!(failures[1].exit_code, 8);
    assert!(failures[1].message.is_empty());
    assert_eq!(failures[1].value, 8);
}
