use crate::fragment::fragment_struct::Fragment;
use std::iter::{FusedIterator, Rev};

/// Mutable iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter_mut_rev()` method.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IterMutRev<'a, T> {
    iter_outer: Rev<std::slice::IterMut<'a, Fragment<T>>>,
    iter_inner: Rev<std::slice::IterMut<'a, T>>,
}

impl<'a, T> IterMutRev<'a, T> {
    pub(crate) fn new(fragments: &'a mut [Fragment<T>]) -> Self {
        let mut iter_outer = fragments.iter_mut().rev();
        let iter_inner = iter_outer
            .next()
            .map(|x| x.iter_mut())
            .unwrap_or([].iter_mut())
            .rev();
        Self {
            iter_outer,
            iter_inner,
        }
    }

    fn next_fragment(&mut self) -> Option<&'a mut T> {
        match self.iter_outer.next() {
            Some(f) => {
                self.iter_inner = f.iter_mut().rev();
                self.next()
            }
            None => None,
        }
    }
}

impl<'a, T> Iterator for IterMutRev<'a, T> {
    type Item = &'a mut T;

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

impl<T> FusedIterator for IterMutRev<'_, T> {}

#[cfg(test)]
mod tests {
    use crate::{test_all_growth_types, Growth, SplitVec};
    use orx_pinned_vec::PinnedVec;

    #[test]
    fn iter_mut() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            let n = 564;
            let stdvec: Vec<_> = (0..n).collect();
            vec.extend(stdvec);

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
}
