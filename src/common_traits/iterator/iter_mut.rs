use crate::fragment::fragment_struct::Fragment;
use std::iter::FusedIterator;

/// Mutable iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter_mut()` method.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IterMut<'a, T> {
    pub(crate) fragments: &'a mut [Fragment<T>],
    pub(crate) f: usize,
    pub(crate) i: usize,
    ptr: *mut T,
}
impl<'a, T> IterMut<'a, T> {
    pub(crate) fn new(fragments: &'a mut [Fragment<T>]) -> Self {
        let f = 0;
        let i = 0;
        let ptr = fragments
            .first_mut()
            .and_then(|x| x.get_as_mut_ptr())
            .unwrap_or(std::ptr::null_mut());
        Self {
            fragments,
            f,
            i,
            ptr,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.fragments[self.f].len() {
            let val = Some(unsafe { &mut *self.ptr });
            self.i += 1;
            self.ptr = unsafe { self.ptr.add(1) };
            val
        } else if self.f == self.fragments.len() - 1 {
            None
        } else {
            self.f += 1;
            self.i = 0;
            self.ptr = self.fragments[self.f]
                .get_as_mut_ptr()
                .unwrap_or(std::ptr::null_mut());
            self.next()
        }
    }
}
impl<T> ExactSizeIterator for IterMut<'_, T> {
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
impl<T> FusedIterator for IterMut<'_, T> {}

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
    fn iter_len() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            let n = 564;
            let n1 = 25;
            let n2 = 184;
            let stdvec: Vec<_> = (0..n).collect();
            vec.extend(stdvec);

            let mut iter = vec.iter_mut();

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
