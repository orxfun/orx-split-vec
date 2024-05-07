use crate::{test_all_growth_types, Growth, SplitVec};
use orx_pinned_vec::PinnedVec;

#[test]
fn iter() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        let n = 564;
        let stdvec: Vec<_> = (0..n).collect();
        vec.extend(stdvec);

        for (i, x) in vec.iter().enumerate() {
            assert_eq!(i, *x);
        }
    }
    test_all_growth_types!(test);
}

#[test]
fn iter_empty_split_vec() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        vec.clear();
        let mut iter = vec.iter();
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

        let mut iter = vec.iter();
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

        assert_eq!(vec![0, 1], vec.iter().copied().collect::<Vec<_>>());
    }
    test_all_growth_types!(test);
}

#[test]
fn clone() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        let n = 564;
        let stdvec: Vec<_> = (0..n).collect();
        vec.extend(stdvec);

        let iter1 = vec.iter();
        let iter2 = iter1.clone();

        for (i, (a, b)) in iter1.zip(iter2).enumerate() {
            assert_eq!(i, *a);
            assert_eq!(i, *b);
        }
    }
    test_all_growth_types!(test);
}

#[test]
fn all() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        let n = 564;
        let stdvec: Vec<_> = (0..n).collect();
        vec.extend(stdvec);

        assert!(vec.iter().all(|x| *x as isize >= -1));
        assert!(!vec.iter().all(|x| *x < 357));
    }
    test_all_growth_types!(test);
}

#[test]
fn any() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        let n = 564;
        let stdvec: Vec<_> = (0..n).collect();
        vec.extend(stdvec);

        assert!(!vec.iter().any(|x| *x as isize <= -1));
        assert!(vec.iter().any(|x| *x < 357));
    }
    test_all_growth_types!(test);
}

#[test]
fn fold() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        let n = 564;
        let stdvec: Vec<_> = (0..n).collect();
        vec.extend(stdvec);

        let sum = vec.iter().fold(0isize, |x, b| {
            if b % 2 == 0 {
                x + *b as isize
            } else {
                x - *b as isize
            }
        });

        let expected = (0..n).filter(|x| x % 2 == 0).sum::<usize>() as isize
            - (0..n).filter(|x| x % 2 == 1).sum::<usize>() as isize;

        assert_eq!(sum, expected);
    }
    test_all_growth_types!(test);
}
