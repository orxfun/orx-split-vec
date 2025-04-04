#![cfg(target_pointer_width = "32")]
use orx_split_vec::{SplitVec, *};

const LEN: usize = 1 << 14;

#[test]
fn grow_with_default_growth() {
    grow_split_vec(SplitVec::new());
}

#[test]
fn grow_with_doubling() {
    grow_split_vec(SplitVec::with_doubling_growth());
    grow_split_vec(SplitVec::with_doubling_growth_and_max_concurrent_capacity());
}

#[test]
fn grow_with_recursive() {
    grow_split_vec(SplitVec::with_recursive_growth());
    grow_split_vec(SplitVec::with_recursive_growth_and_max_concurrent_capacity());
}

#[test]
fn grow_with_linear() {
    grow_split_vec(SplitVec::with_linear_growth(10));
    grow_split_vec(SplitVec::with_linear_growth(28));
}

#[test]
#[should_panic]
fn grow_with_linear_panics_when_capacity_overflows() {
    grow_split_vec(SplitVec::with_linear_growth(29));
}

fn grow_split_vec<G: Growth>(mut vec: SplitVec<u8, G>) {
    assert!(vec.is_empty());

    for i in 0..LEN {
        vec.push(i as u8);
    }

    assert_eq!(vec.len(), LEN);

    for i in 0..LEN {
        assert_eq!(vec.get(i), Some(&(i as u8)));
    }
}
