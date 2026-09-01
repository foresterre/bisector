use super::{super::*, *};

#[yare::parameterized(
    one_to_ten = { input_1_to_10 },
    one = { input_1 },
    empty = { input_empty },
)]

fn new_bisector(input: fn() -> Vec<u32>) {
    let values = input();
    let bisect = Bisector::new(&values);

    assert_eq!(values.len(), bisect.view().len());
}

// Zero elements test
// ------------------

#[test]
fn bisect_on_empty_view_converge_to_left() {
    let values = input_empty();
    let bisector = Bisector::new(&values);

    // These indices don't make sense in the common case, since index 0 would be the first element,
    // but here our slice is empty, so there is no index zero.
    let start_from = Indices::new(0, 0);

    // Yet, because we have index.left == index.right we will get back a step with a None result.
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), start_from);

    assert_eq!(step.indices, start_from);
    assert!(step.result.is_none())
}

#[test]
fn bisect_on_empty_view_converge_to_right() {
    let values = input_empty();
    let bisector = Bisector::new(&values);

    // These indices don't make sense in the common case, since index 0 would be the first element,
    // but here our slice is empty, so there is no index zero.
    let start_from = Indices::new(0, 0);

    // Yet, because we have index.left == index.right we will get back a step with a None result.
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Right(value), start_from);

    assert_eq!(step.indices, start_from);
    assert!(step.result.is_none())
}

// One element test
// -----------------

#[test]
fn bisect_on_view_with_one_element_converge_to_left() {
    let values = input_1();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // Our only element is still part of the view (left = 0, right = |values| = 1), so it is evaluated
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), start_from);

    assert_eq!(step.indices, Indices::new(0, 0));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 1);

    // Now the view is empty, so we're converged
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), step.indices);

    assert_eq!(step.indices, Indices::new(0, 0));
    assert!(step.result.is_none());
}

#[test]
fn bisect_on_view_with_one_element_converge_to_right() {
    let values = input_1();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // Our only element is still part of the view (left = 0, right = |values| = 1), so it is evaluated
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Right(value), start_from);

    assert_eq!(step.indices, Indices::new(1, 1));
    assert_eq!(step.result.unwrap().unwrap_converge_right(), 1);

    // Now the view is empty, so we're converged
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Right(value), step.indices);

    assert_eq!(step.indices, Indices::new(1, 1));
    assert!(step.result.is_none());
}

// Many elements tests
// -------------------

#[test]
fn bisect_on_view_with_many_elements_converge_to_left() {
    let values = input_1_to_10();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // In this test case, we'll manually step through the bisection (i.e. without a loop)

    // (1) We use the default starting indices (i.e. left = 0, right = |values| = 10;
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), start_from);

    // We converge to the left, so our view of values will be halved to the left half.
    assert_eq!(step.indices, Indices::new(0, 5));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 6);

    // (2) Now we use the next indices produced by step, to progress our bisection: step.indices
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), step.indices);

    // We converge to the left, so our view of values will be halved further to the left half.
    assert_eq!(step.indices, Indices::new(0, 2));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 3);

    // (3) Step further
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), step.indices);

    assert_eq!(step.indices, Indices::new(0, 1));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 2);

    // (4) Step further
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), step.indices);

    assert_eq!(step.indices, Indices::new(0, 0));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 1);

    // (5) Step a one more time to check we are at the end
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), step.indices);

    assert_eq!(step.indices, Indices::new(0, 0));
    assert!(step.result.is_none());
}

#[test]
fn bisect_on_view_with_many_elements_converge_to_right() {
    let values = input_1_to_10();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // In this test case, we'll manually step through the bisection (i.e. without a loop)

    // (1) We use the default starting indices (i.e. left = 0, right = |values| = 10;
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Right(value), start_from);

    // We converge to the right, so our view of values will be halved to the right half.
    assert_eq!(step.indices, Indices::new(6, 10));
    assert_eq!(step.result.unwrap().unwrap_converge_right(), 6);

    // (2) Now we use the next indices produced by step, to progress our bisection: step.indices
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Right(value), step.indices);

    // We converge to the right, so our view of values will be halved further to the right half.
    assert_eq!(step.indices, Indices::new(9, 10));
    assert_eq!(step.result.unwrap().unwrap_converge_right(), 9);

    // (3) Step further: the last element is part of the view as well, so it is evaluated too
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Right(value), step.indices);

    assert_eq!(step.indices, Indices::new(10, 10));
    assert_eq!(step.result.unwrap().unwrap_converge_right(), 10);

    // (4) Step a one more time to check we are at the end
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Right(value), step.indices);

    assert_eq!(step.indices, Indices::new(10, 10));
    assert!(step.result.is_none());
}

#[test]
fn bisect_on_view_with_many_elements_converge_zig_zag() {
    let values = input_1_to_10();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    // In this test case, we'll manually step through the bisection (i.e. without a loop)

    // (1) We use the default starting indices (i.e. left = 0, right = |values| = 10;
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), start_from);

    // We converge to the left, so our view of values will be halved to the left half.
    assert_eq!(step.indices, Indices::new(0, 5));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 6);

    // (2) Now we use the next indices produced by step, to progress our bisection: step.indices
    //      Because we zig-zag, we'll now converge to the right
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Right(value), step.indices);

    // We converge to the right, so our view of values will be halved to the right half of our previous
    // view.
    assert_eq!(step.indices, Indices::new(3, 5));
    assert_eq!(step.result.unwrap().unwrap_converge_right(), 3);

    // (3) Step further: zig-zag left
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), step.indices);

    assert_eq!(step.indices, Indices::new(3, 4));
    assert_eq!(step.result.unwrap().unwrap_converge_left(), 5);

    // (4) Step further: left again, now only one element is left in the view
    let final_step: Step<u32, u32> =
        bisector.bisect(|&value| ConvergeTo::Left(value), step.indices);

    assert_eq!(final_step.indices, Indices::new(3, 3));
    assert_eq!(final_step.result.unwrap().unwrap_converge_left(), 4);

    // (5a) Step a one more time to check we are at the end: left
    let step: Step<u32, u32> =
        bisector.bisect(|&value| ConvergeTo::Left(value), final_step.indices);

    assert_eq!(step.indices, Indices::new(3, 3));
    assert!(step.result.is_none());

    // (5b) Step a one more time to check we are at the end: right
    let step: Step<u32, u32> =
        bisector.bisect(|&value| ConvergeTo::Right(value), final_step.indices);

    assert_eq!(step.indices, Indices::new(3, 3));
    assert!(step.result.is_none());
}

#[test]
fn bisect_on_view_with_many_elements_re_use_same_indices_means_no_progress() {
    let values = input_1_to_10();
    let bisector = Bisector::new(&values);

    let start_from = Indices::from_bisector(&bisector);

    let expected_next_indices = Indices::new(0, 5);
    let expected_output_value = 6;

    // In this test case, we'll manually step through the bisection (i.e. without a loop)
    // We'll show no progress is made when the same input indices are used (i.e. the bisector
    //  does not store the progress as internal state).

    // (1) We use the default starting indices (i.e. left = 0, right = |values| = 10;
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), start_from);

    // We converge to the left, so our view of values will be halved to the left half.
    assert_eq!(step.indices, expected_next_indices);
    assert_eq!(
        step.result.unwrap().unwrap_converge_left(),
        expected_output_value
    );

    // (2) Now we use the the same starting indices, instead of the progressed indices from the previous
    //  step (`start_from` instead of `step.indices`)
    let step: Step<u32, u32> = bisector.bisect(|&value| ConvergeTo::Left(value), start_from);

    assert_eq!(step.indices, expected_next_indices);
    assert_eq!(
        step.result.unwrap().unwrap_converge_left(),
        expected_output_value
    );
}
