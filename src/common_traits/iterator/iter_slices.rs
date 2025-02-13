use crate::{
    range_helpers::{range_end, range_start},
    Fragment, Growth, SplitVec,
};
use core::{iter::FusedIterator, ops::RangeBounds};
use orx_pinned_vec::PinnedVec;

pub struct IterSlices<'a, T> {
    fragments: &'a [Fragment<T>],
    sf: usize,
    si: usize,
    si_end: usize,
    ef: usize,
    ei: usize,
    f: usize,
}

impl<'a, T> Default for IterSlices<'a, T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T> IterSlices<'a, T> {
    fn empty() -> Self {
        Self {
            fragments: &[],
            sf: 0,
            si: 0,
            si_end: 0,
            ef: 0,
            ei: 0,
            f: 1,
        }
    }

    fn single_slice(fragments: &'a [Fragment<T>], f: usize, begin: usize, end: usize) -> Self {
        Self {
            fragments,
            sf: f,
            si: begin,
            si_end: end,
            ef: f,
            ei: 0,
            f,
        }
    }

    pub(crate) fn new(
        split_vec: &'a SplitVec<T, impl Growth>,
        range: impl RangeBounds<usize>,
    ) -> Self {
        let a = range_start(&range);
        let b = range_end(&range, split_vec.len());

        match b.saturating_sub(a) {
            0 => Self::empty(),
            _ => match split_vec.get_fragment_and_inner_indices(a) {
                None => Self::empty(),
                Some((sf, si)) => match split_vec.get_fragment_and_inner_indices(b - 1) {
                    None => Self::empty(),
                    Some((ef, ei)) => {
                        let fragments = &split_vec.fragments;
                        match sf == ef {
                            true => Self::single_slice(fragments, sf, si, ei + 1),
                            false => Self {
                                fragments,
                                sf,
                                si,
                                si_end: fragments[sf].len(),
                                ef,
                                ei,
                                f: sf,
                            },
                        }
                    }
                },
            },
        }
    }
}

impl<'a, T> Iterator for IterSlices<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        match self.f {
            f if f == self.sf => {
                self.f += 1;
                Some(&self.fragments[self.sf][self.si..self.si_end])
            }
            f if f < self.ef => {
                self.f += 1;
                Some(&self.fragments[f])
            }
            f if f == self.ef => {
                self.f += 1;
                Some(&self.fragments[self.ef][..=self.ei])
            }
            _ => None,
        }
    }
}

impl<'a, T> FusedIterator for IterSlices<'a, T> {}
