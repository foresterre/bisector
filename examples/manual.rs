use bisector::{Bisector, ConvergeTo, Indices, Step};

fn main() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // In this example, we'll manually step through the bisection (i.e. without a loop).

    // (1) We use the default starting indices (i.e. left = 0, right = |values| - 1 = 9);
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), start_from);

    // We converge to the left, so our view of values will be halved to the left half.
    assert_eq!(step.indices, Indices::new(0, 4));
    assert_eq!(step.result.unwrap().try_into_left().unwrap(), 5);

    // (2) Now we use the next indices produced by step, to progress our bisection: step.indices
    //      Because we zig-zag, we'll now converge to the right
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Right(value), step.indices);

    // We converge to the right, so our view of values will be halved to the right half of our previous
    // view.
    assert_eq!(step.indices, Indices::new(3, 4));
    assert_eq!(step.result.unwrap().try_into_right().unwrap(), 3);

    // (3) Step further: zig-zag left
    let final_step: Step<u32, u32> =
        bisector.bisect(|&value| ConvergeTo::Left(value), step.indices);

    assert_eq!(final_step.indices, Indices::new(3, 3));
    assert_eq!(final_step.result.unwrap().try_into_left().unwrap(), 4);

    // (4a) Step a one more time to check we are at the end: left
    let step: Step<u32, u32> =
        bisector.bisect(|&value| ConvergeTo::Left(value), final_step.indices);

    assert_eq!(step.indices, Indices::new(3, 3));
    assert!(step.result.is_none());

    // (4b) Step a one more time to check we are at the end: right
    let step: Step<u32, u32> =
        bisector.bisect(|&value| ConvergeTo::Right(value), final_step.indices);

    assert_eq!(step.indices, Indices::new(3, 3));
    assert!(step.result.is_none());
}
