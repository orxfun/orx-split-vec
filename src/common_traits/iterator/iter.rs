use crate::fragment::fragment_struct::Fragment;
use std::iter::FusedIterator;

/// Iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter()` method.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a, T> {
    pub(crate) fragments: &'a [Fragment<T>],
    pub(crate) f: usize,
    pub(crate) i: usize,
}
impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(fragments: &'a [Fragment<T>]) -> Self {
        let f = 0;
        let i = 0;
        Self { fragments, f, i }
    }
}
impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Self {
            fragments: self.fragments,
            f: self.f,
            i: self.i,
        }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.fragments[self.f].len() {
            let val = Some(unsafe { self.fragments.get_unchecked(self.f).get_unchecked(self.i) });
            self.i += 1;
            val
        } else {
            self.f += 1;
            self.i = 1;
            self.fragments.get(self.f).and_then(|f| f.get(0))
        }
    }
}
impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.fragments
            .iter()
            .skip(self.f + 1)
            .map(|x| x.len())
            .sum::<usize>()
            + if self.f == self.fragments.len() || self.i == self.fragments[self.f].len() {
                0
            } else {
                self.fragments[self.f].len() - self.i
            }
    }
}
impl<T> FusedIterator for Iter<'_, T> {}

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

            for (i, x) in vec.iter().enumerate() {
                assert_eq!(i, *x);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn iter_len() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            let n = 564;
            let n1 = 25;
            let n2 = 184;
            let stdvec: Vec<_> = (0..n).collect();
            vec.extend(stdvec);

            let mut iter = vec.iter();

            for _ in 0..n1 {
                _ = iter.next();
            }
            assert_eq!(n - n1, iter.len());

            for _ in 0..n2 {
                _ = iter.next();
            }
            assert_eq!(n - n1 - n2, iter.len());
        }

        test_all_growth_types!(test);
    }
}
