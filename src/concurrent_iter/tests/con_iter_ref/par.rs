use crate::*;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use orx_parallel::*;
use std::format;
use test_case::test_matrix;

#[cfg(miri)]
const N: usize = 125;
#[cfg(not(miri))]
const N: usize = 4735;

fn new_vec<G: Growth>(
    mut vec: SplitVec<String, G>,
    n: usize,
    elem: impl Fn(usize) -> String,
) -> SplitVec<String, G> {
    for i in 0..n {
        vec.push(elem(i));
    }
    vec
}

#[test_matrix(
    [SplitVec::with_doubling_growth_and_fragments_capacity(16), SplitVec::with_linear_growth_and_fragments_capacity(10, 33)],
    [0, 1, N],
    [0, 1, 4]
)]
fn abc_par_map_filter_collect<G: ParGrowth>(mut vec: SplitVec<String, G>, n: usize, nt: usize) {
    vec = new_vec(vec, n, |x| (x + 10).to_string());

    let expected: Vec<_> = vec
        .iter()
        .map(|x| format!("{}!", x))
        .filter(|x| !x.starts_with('1'))
        .collect();

    let result: Vec<_> = vec
        .par()
        .num_threads(nt)
        .map(|x| format!("{}!", x))
        .filter(|x| !x.starts_with('1'))
        .collect();

    assert_eq!(expected, result);
}
