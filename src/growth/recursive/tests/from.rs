use crate::*;
use alloc::vec::Vec;

fn validate_clone(original: Vec<usize>, split_vec: SplitVec<usize, Recursive>) {
    assert_eq!(split_vec, &original);
    assert!(original.capacity() <= split_vec.capacity());
}

#[test]
fn from_vec_medium() {
    for len in 0..135 {
        let vec: Vec<_> = (0..len).collect();
        let split_vec: SplitVec<_, Recursive> = vec.clone().into();
        validate_clone(vec, split_vec);
    }
}

#[test]
fn from_doubling() {
    for len in 0..135 {
        let vec: Vec<_> = (0..len).collect();
        let split_vec_doubling: SplitVec<_, Doubling> = vec.clone().into();
        let split_vec: SplitVec<_, Recursive> = split_vec_doubling.into();
        validate_clone(vec, split_vec);
    }
}
