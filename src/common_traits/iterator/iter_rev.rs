use crate::fragment::fragment_struct::Fragment;
use std::iter::{FusedIterator, Rev};

/// Iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter_rev()` method.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IterRev<'a, T> {
    iter_outer: Rev<std::slice::Iter<'a, Fragment<T>>>,
    iter_inner: Rev<std::slice::Iter<'a, T>>,
}

impl<'a, T> IterRev<'a, T> {
    pub(crate) fn new(fragments: &'a [Fragment<T>]) -> Self {
        let mut iter_outer = fragments.iter().rev();
        let iter_inner = iter_outer
            .next()
            .map(|x| x.iter().rev())
            .unwrap_or([].iter().rev());
        Self {
            iter_outer,
            iter_inner,
        }
    }

    fn next_fragment(&mut self) -> Option<&'a T> {
        match self.iter_outer.next() {
            Some(f) => {
                self.iter_inner = f.iter().rev();
                self.next()
            }
            None => None,
        }
    }
}

impl<'a, T> Clone for IterRev<'a, T> {
    fn clone(&self) -> Self {
        Self {
            iter_outer: self.iter_outer.clone(),
            iter_inner: self.iter_inner.clone(),
        }
    }
}

impl<'a, T> Iterator for IterRev<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let next_element = self.iter_inner.next();
        if next_element.is_some() {
            next_element
        } else {
            self.next_fragment()
        }
    }
}

impl<T> FusedIterator for IterRev<'_, T> {}

#[cfg(test)]
mod tests {
    use crate::{test_all_growth_types, Growth, SplitVec};
    use orx_pinned_vec::PinnedVec;

    #[test]
    fn iter() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            let n = 564;
            let stdvec: Vec<_> = (0..n).collect();
            vec.extend(stdvec);

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

            assert_eq!(vec![1, 0], vec.iter_rev().copied().collect::<Vec<_>>());
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn clone() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            let n = 564;
            let stdvec: Vec<_> = (0..n).collect();
            vec.extend(stdvec);

            let iter1 = vec.iter_rev();
            let iter2 = iter1.clone();

            for (i, (a, b)) in iter1.zip(iter2).enumerate() {
                assert_eq!(n - i - 1, *a);
                assert_eq!(n - i - 1, *b);
            }
        }
        test_all_growth_types!(test);
    }
}
