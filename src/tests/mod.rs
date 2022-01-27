mod bisect;
mod bisect_complex;
mod external_program;
mod indices;

pub(in crate::tests) fn input_1_to_10() -> Vec<u32> {
    (1..=10).collect()
}

pub(in crate::tests) fn input_1() -> Vec<u32> {
    vec![1]
}

pub(in crate::tests) fn input_empty() -> Vec<u32> {
    vec![]
}
