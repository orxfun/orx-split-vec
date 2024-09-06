use crate::{test_all_growth_types, Growth, SplitVec};
use alloc::vec::Vec;
use orx_pinned_vec::PinnedVec;

#[test]
fn iter() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        let n = 564;
        let std_vec: Vec<_> = (0..n).collect();
        vec.extend(std_vec);

        for (i, x) in vec.iter_rev().enumerate() {
            assert_eq!(n - i - 1, *x);
        }
    }
    test_all_growth_types!(test);
}

#[test]
fn iter_empty_split_vec() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        vec.clear();
        let mut iter = vec.iter_rev();
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }
    test_all_growth_types!(test);
}

#[test]
fn iter_empty_first_fragment() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        vec.clear();
        vec.push(0);
        _ = vec.pop();
        assert!(vec.is_empty());

        let mut iter = vec.iter_rev();
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }
    test_all_growth_types!(test);
}

#[test]
fn iter_one_fragment() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        vec.clear();
        vec.push(0);
        vec.push(1);

        assert_eq!(
            alloc::vec![1, 0],
            vec.iter_rev().copied().collect::<Vec<_>>()
        );
    }
    test_all_growth_types!(test);
}

#[test]
fn clone() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        let n = 564;
        let std_vec: Vec<_> = (0..n).collect();
        vec.extend(std_vec);

        let iter1 = vec.iter_rev();
        let iter2 = iter1.clone();

        for (i, (a, b)) in iter1.zip(iter2).enumerate() {
            assert_eq!(n - i - 1, *a);
            assert_eq!(n - i - 1, *b);
        }
    }
    test_all_growth_types!(test);
}
