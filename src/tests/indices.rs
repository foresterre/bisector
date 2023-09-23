use super::{super::*, *};

#[yare::parameterized(
    one_to_ten = { input_1_to_10, (0, 9) },
    one = { input_1, (0, 0) },
)]
fn create_starting_indices_from_bisector(
    input: fn() -> Vec<u32>,
    indices_expected: (usize, usize),
) {
    let values = input();
    let bisector = Bisector::new(&values);

    let indices = Indices::from_bisector(&bisector);

    let (left_expected, right_expected) = indices_expected;

    assert_eq!(indices.left, left_expected);
    assert_eq!(indices.right, right_expected);
}

#[test]
#[should_panic]
fn creating_starting_indices_from_bisector_with_empty_slice_panics() {
    let values = input_empty();
    let bisector = Bisector::new(&values);

    let _ = Indices::from_bisector(&bisector);
}

#[test]
fn create_indices_with_new() {
    let indices = Indices::new(0, 1);

    assert_eq!(indices.left, 0);
    assert_eq!(indices.right, 1);
}

#[yare::parameterized(
    one_to_ten = { input_1_to_10, (0, 9) },
    one = { input_1, (0, 0) },
)]
fn create_starting_indices_try_from_bisector(
    input: fn() -> Vec<u32>,
    indices_expected: (usize, usize),
) {
    let values = input();
    let bisector = Bisector::new(&values);

    let indices = Indices::try_from_bisector(&bisector).unwrap();

    let (left_expected, right_expected) = indices_expected;

    assert_eq!(indices.left, left_expected);
    assert_eq!(indices.right, right_expected);
}

#[test]
fn creating_starting_indices_try_from_bisector_with_empty_slice_should_error() {
    let values = input_empty();
    let bisector = Bisector::new(&values);

    let result = Indices::try_from_bisector(&bisector);

    assert_eq!(result.unwrap_err(), EmptySliceError);
}

#[yare::parameterized(
    zeros = { 0, 0, 0 },
    zero_one = { 0, 1, 0 },
    zero_two = { 0, 2, 1 },
    zero_three = { 0, 3, 1 },
    zero_four = { 0, 4, 2 },
    one_one = { 1, 1, 1 },
    one_two = { 1, 2, 1 },
)]
fn middle_of_indices(left: usize, right: usize, expected_middle: usize) {
    let indices = Indices::new(left, right);

    let middle = indices.middle();
    assert_eq!(middle, expected_middle);
}

#[yare::parameterized(
    one_zero = { 1, 0 },
    two_zero = { 2, 0 },
    two_one = { 2, 1 },
)]
#[should_panic]
fn middle_of_invalid_indices(left: usize, right: usize) {
    let indices = Indices::new(left, right);

    indices.middle();
}
