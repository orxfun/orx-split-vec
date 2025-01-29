use crate::{test_all_growth_types, Growth, SplitVec};
use alloc::vec::Vec;
use orx_pinned_vec::*;

#[test]
fn iter_mut() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        let n = 564;
        let std_vec: Vec<_> = (0..n).collect();
        vec.extend(std_vec);

        let mut iter = vec.iter_mut_rev().enumerate();
        #[allow(clippy::while_let_on_iterator)]
        while let Some((i, x)) = iter.next() {
            *x += i;
        }

        for x in vec.iter() {
            assert_eq!(n - 1, *x);
        }
    }

    test_all_growth_types!(test);
}

#[test]
fn iter_empty_split_vec() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        vec.clear();
        let mut iter = vec.iter_mut_rev();
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

        let mut iter = vec.iter_mut_rev();
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

        let mut iter = vec.iter_mut_rev();
        assert_eq!(Some(&mut 1), iter.next());
        assert_eq!(Some(&mut 0), iter.next());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }
    test_all_growth_types!(test);
}
