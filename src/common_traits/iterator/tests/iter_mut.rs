use crate::{Growth, SplitVec, test_all_growth_types};
use alloc::vec::Vec;
use orx_pinned_vec::*;

#[test]
fn iter_mut() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        #[cfg(not(miri))]
        let n = 564;
        #[cfg(miri)]
        let n = 37;

        let std_vec: Vec<_> = (0..n).collect();
        vec.extend(std_vec);

        let mut iter = vec.iter_mut();
        #[allow(clippy::while_let_on_iterator)]
        while let Some(x) = iter.next() {
            *x *= 10;
        }

        for (i, x) in vec.iter().enumerate() {
            assert_eq!(i * 10, *x);
        }

        for x in vec.iter_mut() {
            *x += 10;
        }
        for (i, x) in vec.iter().enumerate() {
            assert_eq!(i * 10 + 10, *x);
        }
    }

    test_all_growth_types!(test);
}

#[test]
fn iter_empty_split_vec() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        vec.clear();
        let mut iter = vec.iter_mut();
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

        let mut iter = vec.iter_mut();
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

        let mut iter = vec.iter_mut();
        assert_eq!(Some(&mut 0), iter.next());
        assert_eq!(Some(&mut 1), iter.next());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }
    test_all_growth_types!(test);
}
