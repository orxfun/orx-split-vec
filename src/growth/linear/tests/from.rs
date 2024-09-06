use crate::*;
use alloc::vec::Vec;

fn validate_clone(original: Vec<usize>, split_vec: SplitVec<usize, Doubling>) {
    assert_eq!(split_vec, &original);
    assert!(original.capacity() <= split_vec.capacity());
}

#[test]
fn from_vec_medium() {
    for len in 0..135 {
        let vec: Vec<_> = (0..len).collect();
        let split_vec: SplitVec<_, Doubling> = vec.clone().into();
        validate_clone(vec, split_vec);
    }
}
