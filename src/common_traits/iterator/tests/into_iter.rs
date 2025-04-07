use crate::{Growth, SplitVec, test_all_growth_types};
use alloc::vec::Vec;
use orx_pinned_vec::*;

#[test]
fn iter() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        #[cfg(not(miri))]
        let n = 564;
        #[cfg(miri)]
        let n = 37;

        let std_vec: Vec<_> = (0..n).collect();
        vec.extend(std_vec);

        for (i, x) in vec.into_iter().enumerate() {
            assert_eq!(i, x);
        }
    }
    test_all_growth_types!(test);
}

#[test]
fn iter_empty_split_vec() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        vec.clear();
        let mut iter = vec.into_iter();
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

        let mut iter = vec.into_iter();
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

        assert_eq!(alloc::vec![0, 1], vec.into_iter().collect::<Vec<_>>());
    }
    test_all_growth_types!(test);
}

#[test]
fn clone() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        #[cfg(not(miri))]
        let n = 564;
        #[cfg(miri)]
        let n = 37;

        let std_vec: Vec<_> = (0..n).collect();
        vec.extend(std_vec);

        let iter1 = vec.into_iter();
        let iter2 = iter1.clone();

        for (i, (a, b)) in iter1.zip(iter2).enumerate() {
            assert_eq!(i, a);
            assert_eq!(i, b);
        }
    }
    test_all_growth_types!(test);
}
