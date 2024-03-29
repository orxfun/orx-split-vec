use crate::fragment::fragment_struct::Fragment;
use std::iter::FusedIterator;

/// Mutable iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter_mut()` method.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IterMut<'a, T> {
    iter_outer: std::slice::IterMut<'a, Fragment<T>>,
    iter_inner: std::slice::IterMut<'a, T>,
}

impl<'a, T> IterMut<'a, T> {
    pub(crate) fn new(fragments: &'a mut [Fragment<T>]) -> Self {
        let mut iter_outer = fragments.iter_mut();
        let iter_inner = iter_outer
            .next()
            .map(|x| x.iter_mut())
            .unwrap_or([].iter_mut());
        Self {
            iter_outer,
            iter_inner,
        }
    }

    fn next_fragment(&mut self) -> Option<&'a mut T> {
        match self.iter_outer.next() {
            Some(f) => {
                self.iter_inner = f.iter_mut();
                self.next()
            }
            None => None,
        }
    }
}

impl<T> FusedIterator for IterMut<'_, T> {}

impl<'a, T> Iterator for IterMut<'a, T> {
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
}
