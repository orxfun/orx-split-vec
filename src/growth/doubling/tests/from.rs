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

#[test]
fn from_same_as_push() {
    for len in 0..135 {
        let vec: Vec<_> = (0..len).collect();
        let split_vec_from: SplitVec<_, Doubling> = vec.clone().into();
        let mut split_vec_manual = SplitVec::new();
        for item in vec {
            split_vec_manual.push(item);
        }
        assert_eq!(split_vec_from, split_vec_manual);
    }
}
