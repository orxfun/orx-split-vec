use crate::{Growth, SplitVec};
use core::sync::atomic::{AtomicUsize, Ordering};
use orx_pseudo_default::PseudoDefault;

pub struct ConIterSplitVec<T, G>
where
    T: Send + Sync,
    G: Growth,
{
    vec: SplitVec<T, G>,
    counter: AtomicUsize,
}

unsafe impl<T, G> Sync for ConIterSplitVec<T, G>
where
    T: Send + Sync,
    G: Growth,
{
}

unsafe impl<T, G> Send for ConIterSplitVec<T, G>
where
    T: Send + Sync,
    G: Growth,
{
}

impl<T, G> Default for ConIterSplitVec<T, G>
where
    T: Send + Sync,
    G: Growth,
{
    fn default() -> Self {
        Self::new(PseudoDefault::pseudo_default())
    }
}

impl<T, G> Drop for ConIterSplitVec<T, G>
where
    T: Send + Sync,
    G: Growth,
{
    fn drop(&mut self) {
        // TODO: DROP
        // let _iter = self.remaining_into_seq_iter();
    }
}

impl<T, G> ConIterSplitVec<T, G>
where
    T: Send + Sync,
    G: Growth,
{
    pub(super) fn new(vec: SplitVec<T, G>) -> Self {
        Self {
            vec,
            counter: 0.into(),
        }
    }

    pub(super) fn initial_len(&self) -> usize {
        self.vec.len
    }

    fn progress_and_get_begin_idx(&self, number_to_fetch: usize) -> Option<usize> {
        let begin_idx = self.counter.fetch_add(number_to_fetch, Ordering::Relaxed);
        match begin_idx < self.vec.len {
            true => Some(begin_idx),
            _ => None,
        }
    }

    pub(super) fn progress_and_get_chunk_pointers(
        &self,
        chunk_size: usize,
    ) -> Option<(usize, *const T, *const T)> {
        // self.progress_and_get_begin_idx(chunk_size)
        //     .map(|begin_idx| {
        //         let end_idx = (begin_idx + chunk_size).min(self.vec_len).max(begin_idx);
        //         let first = unsafe { self.ptr.add(begin_idx) }; // ptr + begin_idx is in bounds
        //         let last = unsafe { self.ptr.add(end_idx - 1) }; // ptr + end_idx - 1 is in bounds
        //         (begin_idx, first, last)
        //     })
        todo!()
    }

    // fn remaining_into_seq_iter(&mut self) -> VecIntoSeqIter<T> {
    //     // # SAFETY
    //     // null ptr indicates that the data is already taken out of this iterator
    //     // by a consuming method such as `into_seq_iter`
    //     match self.ptr.is_null() {
    //         true => Default::default(),
    //         false => {
    //             let num_taken = self.counter.load(Ordering::Acquire).min(self.vec_len);
    //             let iter = self.slice_into_seq_iter(num_taken, true);
    //             self.ptr = core::ptr::null();
    //             iter
    //         }
    //     }
    // }

    // fn slice_into_seq_iter(&self, num_taken: usize, drop_vec: bool) -> VecIntoSeqIter<T> {
    //     let p = self.ptr;
    //     let completed = num_taken == self.vec.len;

    //     let (first, last, current) = match completed {
    //         true => (p, p, p),
    //         false => {
    //             let first = p;
    //             let last = unsafe { first.add(self.vec.len - 1) }; // self.vec_len is positive here
    //             let current = unsafe { first.add(num_taken) }; // first + num_taken is in bounds
    //             (first, last, current)
    //         }
    //     };

    //     let drop_vec_capacity = drop_vec.then_some(self.vec_cap);
    //     VecIntoSeqIter::new(completed, first, last, current, drop_vec_capacity)
    // }
}
