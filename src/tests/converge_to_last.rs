use super::{super::*, *};

fn last_index_where_compatible(values: &[u32], max_compatible: u32) -> Option<usize> {
    let bisector = Bisector::new(values);

    let mut indices = Indices::try_from_bisector(&bisector).unwrap();
    let mut last_compatible = None;

    while let Step {
        indices: next_indices,
        result: Some(step),
    } = bisector.bisect(
        |&value| {
            if value <= max_compatible {
                ConvergeTo::Right(value)
            } else {
                ConvergeTo::Left(value)
            }
        },
        indices,
    ) {
        if let ConvergeTo::Right(_) = step {
            last_compatible = Some(indices.middle());
        }

        indices = next_indices;
    }

    last_compatible
}

#[yare::parameterized(
    none_compatible = { 0, None },
    first_compatible = { 1, Some(0) },
    halfway_compatible = { 5, Some(4) },
    second_to_last_compatible = { 9, Some(8) },
    last_compatible = { 10, Some(9) },
    all_compatible = { 100, Some(9) },
)]
fn bisect_last_element_is_evaluated(max_compatible: u32, expected: Option<usize>) {
    let values = input_1_to_10();

    assert_eq!(
        last_index_where_compatible(&values, max_compatible),
        expected
    );
}

#[yare::parameterized(
    one_compatible = { 1, Some(0) },
    one_incompatible = { 0, None },
)]
fn bisect_single_element_view_is_evaluated(max_compatible: u32, expected: Option<usize>) {
    let values = input_1();

    assert_eq!(
        last_index_where_compatible(&values, max_compatible),
        expected
    );
}
