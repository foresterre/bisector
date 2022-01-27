use super::{super::*, *};

#[test]
fn try_bisect_on_view_with_many_elements_converge_to_left_ok() {
    let values = input_1_to_10();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // In this test case, we'll manually step through the bisection (i.e. without a loop)

    // (1) We use the default starting indices (i.e. left = 0, right = |values| - 1 = 9;
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Left(value)), start_from);
    let step = step.unwrap();

    // We converge to the left, so our view of values will be halved to the left half.
    assert_eq!(step.indices, Indices::new(0, 4));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 5);

    // (2) Now we use the next indices produced by step, to progress our bisection: step.indices
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Left(value)), step.indices);
    let step = step.unwrap();

    // We converge to the left, so our view of values will be halved further to the left half.
    assert_eq!(step.indices, Indices::new(0, 2));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 3);

    // (3) Step further
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Left(value)), step.indices);
    let step = step.unwrap();

    assert_eq!(step.indices, Indices::new(0, 1));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 2);

    // (4) Step further
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Left(value)), step.indices);
    let step = step.unwrap();

    assert_eq!(step.indices, Indices::new(0, 0));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 1);

    // (5) Step a one more time to check we are at the end
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Left(value)), step.indices);
    let step = step.unwrap();

    assert_eq!(step.indices, Indices::new(0, 0));
    assert!(step.result.is_none());
}

#[test]
fn try_bisect_on_view_with_many_elements_converge_to_right_ok() {
    let values = input_1_to_10();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // In this test case, we'll manually step through the bisection (i.e. without a loop)

    // (1) We use the default starting indices (i.e. left = 0, right = |values| - 1 = 9;
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Right(value)), start_from);
    let step = step.unwrap();

    // We converge to the right, so our view of values will be halved to the right half.
    assert_eq!(step.indices, Indices::new(5, 9));
    assert_eq!(step.result.unwrap().unwrap_converge_right(), 5);

    // (2) Now we use the next indices produced by step, to progress our bisection: step.indices
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Right(value)), step.indices);
    let step = step.unwrap();

    // We converge to the right, so our view of values will be halved further to the right half.
    assert_eq!(step.indices, Indices::new(8, 9));
    assert_eq!(step.result.unwrap().unwrap_converge_right(), 8);

    // (3) Step further
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Right(value)), step.indices);
    let step = step.unwrap();

    assert_eq!(step.indices, Indices::new(9, 9));
    assert_eq!(step.result.unwrap().unwrap_converge_right(), 9);

    // (4) Step a one more time to check we are at the end
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Right(value)), step.indices);
    let step = step.unwrap();

    assert_eq!(step.indices, Indices::new(9, 9));
    assert!(step.result.is_none());
}

#[test]
fn try_bisect_on_view_with_many_elements_converge_zig_zag_ok() {
    let values = input_1_to_10();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // In this test case, we'll manually step through the bisection (i.e. without a loop)

    // (1) We use the default starting indices (i.e. left = 0, right = |values| - 1 = 9;
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Left(value)), start_from);
    let step = step.unwrap();

    // We converge to the left, so our view of values will be halved to the left half.
    assert_eq!(step.indices, Indices::new(0, 4));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 5);

    // (2) Now we use the next indices produced by step, to progress our bisection: step.indices
    //      Because we zig-zag, we'll now converge to the right
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Right(value)), step.indices);
    let step = step.unwrap();

    // We converge to the right, so our view of values will be halved to the right half of our previous
    // view.
    assert_eq!(step.indices, Indices::new(3, 4));
    assert_eq!(step.result.unwrap().unwrap_converge_right(), 3);

    // (3) Step further: zig-zag left
    let final_step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Left(value)), step.indices);
    let final_step = final_step.unwrap();

    assert_eq!(final_step.indices, Indices::new(3, 3));
    assert_eq!(final_step.result.unwrap().unwrap_converge_left(), 4);

    // (4a) Step a one more time to check we are at the end: left
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Left(value)), final_step.indices);
    let step = step.unwrap();

    assert_eq!(step.indices, Indices::new(3, 3));
    assert!(step.result.is_none());

    // (4b) Step a one more time to check we are at the end: right
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Right(value)), final_step.indices);
    let step = step.unwrap();

    assert_eq!(step.indices, Indices::new(3, 3));
    assert!(step.result.is_none());
}

#[test]
fn try_bisect_on_view_with_many_elements_re_use_same_indices_means_no_progress_with_ok() {
    let values = input_1_to_10();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    let expected_next_indices = Indices::new(0, 4);
    let expected_output_value = 5;

    // In this test case, we'll manually step through the bisection (i.e. without a loop)
    // We'll show no progress is made when the same input indices are used (i.e. the bisector
    //  does not store the progress as internal state).

    // (1) We use the default starting indices (i.e. left = 0, right = |values| - 1 = 9;
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Left(value)), start_from);
    let step = step.unwrap();

    // We converge to the left, so our view of values will be halved to the left half.
    assert_eq!(step.indices, expected_next_indices);
    assert_eq!(
        step.result.unwrap().unwrap_converge_left(),
        expected_output_value
    );

    // (2) Now we use the the same starting indices, instead of the progressed indices from the previous
    //  step (`start_from` instead of `step.indices`)
    let step: Result<Step<u32, u32>, ()> =
        bisector.try_bisect(|&value| Ok(ConvergeTo::Left(value)), start_from);
    let step = step.unwrap();

    assert_eq!(step.indices, expected_next_indices);
    assert_eq!(
        step.result.unwrap().unwrap_converge_left(),
        expected_output_value
    );
}

#[test]
fn try_bisect_with_err() {
    let values = input_1_to_10();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // In this test case, we'll manually step through the bisection (i.e. without a loop)

    // (1) We use the default starting indices (i.e. left = 0, right = |values| - 1 = 9;
    let step: Result<Step<u32, u32>, ()> = bisector.try_bisect(|_| Err(()), start_from);

    assert!(step.is_err());
}

#[test]
fn try_bisect_already_converged() {
    let values = input_1();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // This test case is already converged, so we get an Ok result, even though our convergence function
    // always produces an error

    let step: Result<Step<u32, u32>, ()> = bisector.try_bisect(|_| Err(()), start_from);

    assert!(step.is_ok());
}
