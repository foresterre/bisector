use bisector::{Bisector, ConvergeTo, Indices, Step};

// NB: output held by ConvergeTo does *not* need to be of the same type as
// the value. In this example, it just happens to be the case.
fn f(value: u32) -> ConvergeTo<u32, u32> {
    if value >= 5 && value <= 6 {
        ConvergeTo::Right(value)
    } else {
        ConvergeTo::Left(value)
    }
}

fn main() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let bisector = Bisector::new(&values);
    let mut elements_seen = vec![];
    let mut value = None;

    let mut i = Indices::from_bisector(&bisector);
    while let Step {
        indices,
        result: Some(t),
    } = bisector.bisect(|&v| f(v), i)
    {
        i = indices;

        let val = match t {
            ConvergeTo::Left(l) => l,
            ConvergeTo::Right(r) => r,
        };

        elements_seen.push(val);
        value = Some(val);
    }

    println!("{:?}", elements_seen);
    println!("Final converged to '{}'", value.unwrap());
}
