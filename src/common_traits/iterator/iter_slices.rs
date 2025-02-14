use crate::{
    range_helpers::{range_end, range_start},
    Fragment, Growth, SplitVec,
};
use core::{
    borrow::Borrow,
    iter::FusedIterator,
    ops::RangeBounds,
    slice::{from_raw_parts, from_raw_parts_mut},
};
use orx_pinned_vec::PinnedVec;

// variants

pub trait SliceBorrowKind<'a, T> {
    type FragmentsData;
    type Slice;
    type Ptr;
    type SplitVecBorrow<G>: Borrow<SplitVec<T, G>>
    where
        G: Growth + 'a;

    fn default_fragments_data() -> Self::FragmentsData;

    fn split_vec_fragments_data(
        split_vec: Self::SplitVecBorrow<impl Growth>,
    ) -> Self::FragmentsData;

    fn get_ptr_fi(fragments: &mut Self::FragmentsData, f: usize, i: usize) -> Self::Ptr;

    fn get_slice(ptr: Self::Ptr, len: usize) -> Self::Slice;

    fn fragment_len(fragments: &Self::FragmentsData, f: usize) -> usize;
}

pub struct SliceBorrowAsRef;

impl<'a, T: 'a> SliceBorrowKind<'a, T> for SliceBorrowAsRef {
    type FragmentsData = &'a [Fragment<T>];
    type Slice = &'a [T];
    type Ptr = *const T;
    type SplitVecBorrow<G>
        = &'a SplitVec<T, G>
    where
        G: Growth + 'a;

    fn default_fragments_data() -> Self::FragmentsData {
        &[]
    }

    fn split_vec_fragments_data(
        split_vec: Self::SplitVecBorrow<impl Growth>,
    ) -> Self::FragmentsData {
        &split_vec.fragments
    }

    fn get_ptr_fi(fragments: &mut Self::FragmentsData, f: usize, i: usize) -> Self::Ptr {
        let p = fragments[f].as_ptr();
        unsafe { p.add(i) }
    }

    fn get_slice(ptr: Self::Ptr, len: usize) -> Self::Slice {
        unsafe { from_raw_parts(ptr, len) }
    }

    fn fragment_len(fragments: &Self::FragmentsData, f: usize) -> usize {
        fragments[f].len()
    }
}

pub struct SliceBorrowAsMut;

impl<'a, T: 'a> SliceBorrowKind<'a, T> for SliceBorrowAsMut {
    type FragmentsData = &'a mut [Fragment<T>];
    type Slice = &'a mut [T];
    type Ptr = *mut T;
    type SplitVecBorrow<G>
        = &'a mut SplitVec<T, G>
    where
        G: Growth + 'a;

    fn default_fragments_data() -> Self::FragmentsData {
        &mut []
    }

    fn split_vec_fragments_data(
        split_vec: Self::SplitVecBorrow<impl Growth>,
    ) -> Self::FragmentsData {
        &mut split_vec.fragments
    }

    fn get_ptr_fi(fragments: &mut Self::FragmentsData, f: usize, i: usize) -> Self::Ptr {
        let p = fragments[f].as_mut_ptr();
        unsafe { p.add(i) }
    }
    fn get_slice(ptr: Self::Ptr, len: usize) -> Self::Slice {
        unsafe { from_raw_parts_mut(ptr, len) }
    }

    fn fragment_len(fragments: &Self::FragmentsData, f: usize) -> usize {
        fragments[f].len()
    }
}

// iter

pub struct IterOfSlices<'a, T, K>
where
    K: SliceBorrowKind<'a, T>,
{
    fragments: K::FragmentsData,
    sf: usize,
    si: usize,
    si_end: usize,
    ef: usize,
    ei: usize,
    f: usize,
}

impl<'a, T, K> IterOfSlices<'a, T, K>
where
    K: SliceBorrowKind<'a, T>,
{
    fn empty() -> Self {
        Self {
            fragments: K::default_fragments_data(),
            sf: 0,
            si: 0,
            si_end: 0,
            ef: 0,
            ei: 0,
            f: 1,
        }
    }

    fn single_slice(fragments: K::FragmentsData, f: usize, begin: usize, end: usize) -> Self {
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
        split_vec: K::SplitVecBorrow<impl Growth>,
        range: impl RangeBounds<usize>,
    ) -> Self {
        let vec = split_vec.borrow();
        let a = range_start(&range);
        let b = range_end(&range, vec.len());

        match b.saturating_sub(a) {
            0 => Self::empty(),
            _ => match vec.get_fragment_and_inner_indices(a) {
                None => Self::empty(),
                Some((sf, si)) => match vec.get_fragment_and_inner_indices(b - 1) {
                    None => Self::empty(),
                    Some((ef, ei)) => {
                        let si_end = vec.fragments[sf].len();
                        let fragments = K::split_vec_fragments_data(split_vec);
                        match sf == ef {
                            true => Self::single_slice(fragments, sf, si, ei + 1),
                            false => Self {
                                fragments,
                                sf,
                                si,
                                si_end,
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

    #[inline(always)]
    fn remaining_len(&self) -> usize {
        (1 + self.ef).saturating_sub(self.f)
    }
}

impl<'a, T, K> Default for IterOfSlices<'a, T, K>
where
    K: SliceBorrowKind<'a, T>,
{
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T, K> Iterator for IterOfSlices<'a, T, K>
where
    K: SliceBorrowKind<'a, T>,
{
    type Item = K::Slice;

    fn next(&mut self) -> Option<Self::Item> {
        match self.f {
            f if f == self.sf => {
                self.f += 1;
                let len = self.si_end - self.si;
                let p = K::get_ptr_fi(&mut self.fragments, self.sf, self.si);
                Some(K::get_slice(p, len))
            }
            f if f < self.ef => {
                self.f += 1;
                let len = K::fragment_len(&self.fragments, f);
                let p = K::get_ptr_fi(&mut self.fragments, f, 0);
                Some(K::get_slice(p, len))
            }
            f if f == self.ef => {
                self.f += 1;
                let len = self.ei + 1;
                let p = K::get_ptr_fi(&mut self.fragments, self.ef, 0);
                Some(K::get_slice(p, len))
            }
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining_len();
        (len, Some(len))
    }
}

impl<'a, T, K> FusedIterator for IterOfSlices<'a, T, K> where K: SliceBorrowKind<'a, T> {}

impl<'a, T, K> ExactSizeIterator for IterOfSlices<'a, T, K>
where
    K: SliceBorrowKind<'a, T>,
{
    fn len(&self) -> usize {
        self.remaining_len()
    }
}
