use super::iter_fragment::FragmentIter;
use crate::fragment::fragment_struct::Fragment;
use std::iter::FusedIterator;

/// Iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter()` method.
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IterCon<'a, T> {
    num_fragments: usize,
    last_fragment_len: usize,
    fragments: &'a [Fragment<T>],
    inner: FragmentIter<'a, T>,
    f: usize,
}

impl<'a, T> IterCon<'a, T> {
    pub(crate) fn new(fragments: &'a [Fragment<T>], last_fragment_len: usize) -> Self {
        assert!(!fragments.is_empty());

        let num_fragments = fragments.len();

        let first_fragment_len = match num_fragments {
            0 => 0,
            1 => last_fragment_len,
            _ => fragments[0].capacity(),
        };
        let inner = FragmentIter::new(&fragments[0], first_fragment_len);

        Self {
            num_fragments,
            last_fragment_len,
            fragments,
            inner,
            f: 0,
        }
    }

    fn next_fragment(&mut self) -> Option<&'a T> {
        match self.f + 1 < self.num_fragments {
            true => {
                self.f += 1;
                let fragment_len = match self.f == self.num_fragments - 1 {
                    false => self.fragments[self.f].capacity(),
                    true => self.last_fragment_len,
                };
                self.inner = FragmentIter::new(&self.fragments[self.f], fragment_len);
                self.next()
            }
            false => None,
        }
    }
}

impl<'a, T> Iterator for IterCon<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let next_element = self.inner.next();
        match next_element.is_some() {
            true => next_element,
            false => self.next_fragment(),
        }
    }
}

impl<T> FusedIterator for IterCon<'_, T> {}
