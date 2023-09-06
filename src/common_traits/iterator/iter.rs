use crate::{fragment::fragment_struct::Fragment, Growth, SplitVec};
use std::iter::FusedIterator;

impl<T, G> SplitVec<T, G>
where
    G: Growth,
{
    /// Iterator over the split vec.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(3);
    ///
    /// vec.push(0);
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    ///
    /// let mut iter = vec.iter();
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter<'v, 'i>(&'v self) -> Iter<'i, T>
    where
        'v: 'i,
    {
        Iter {
            fragments: &self.fragments,
            f: 0,
            i: 0,
        }
    }
}

/// Iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter()` method.
#[derive(Debug)]
pub struct Iter<'a, T> {
    pub(crate) fragments: &'a Vec<Fragment<T>>,
    pub(crate) f: usize,
    pub(crate) i: usize,
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
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.fragments[self.f].len() {
            self.f += 1;
            self.i = 1;
            self.fragments.get(self.f).and_then(|f| f.get(0))
        } else {
            self.i += 1;
            self.fragments[self.f].get(self.i - 1)
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
    use super::*;
    use crate::test_all_growth_types;
    use orx_pinned_vec::PinnedVec;

    #[test]
    fn iter() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..1000 {
                vec.push(i);
            }

            let mut iter = vec.iter();
            for i in 0..1000 {
                assert_eq!(1000 - i, iter.len());
                assert_eq!(Some(&i), iter.next());
            }
            assert_eq!(None, iter.next());
        }
        test_all_growth_types!(test);
    }
}
