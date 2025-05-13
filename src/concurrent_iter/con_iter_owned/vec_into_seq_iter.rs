use alloc::vec::Vec;
use core::iter::FusedIterator;
use orx_concurrent_iter::implementations::VecIntoSeqIter;

use crate::SplitVec;

pub struct SplitVecIntoSeqIter2<T>
where
    T: Send + Sync,
{
    pub current: VecIntoSeqIter<T>,
    iters: Vec<VecIntoSeqIter<T>>,
}

impl<T> SplitVecIntoSeqIter2<T>
where
    T: Send + Sync,
{
    pub(super) fn new2(mut iters: impl Iterator<Item = VecIntoSeqIter<T>>) -> Self {
        match iters.next() {
            Some(current) => {
                let mut iters: Vec<_> = iters.collect();
                iters.reverse();
                Self { current, iters }
            }
            None => Self::default(),
        }
    }

    pub(super) fn new(vec: SplitVec<T>, num_taken: usize) -> Self {
        let completed = num_taken == vec.len;
        let mut num_taken = num_taken;
        todo!()
    }

    fn is_completed(&self) -> bool {
        self.iters.is_empty() && self.current.len() == 0
    }

    fn remaining(&self) -> usize {
        match self.is_completed() {
            true => 0,
            false => self.current.len() + self.iters.iter().map(|x| x.len()).sum::<usize>(),
        }
    }
}

impl<T> Default for SplitVecIntoSeqIter2<T>
where
    T: Send + Sync,
{
    fn default() -> Self {
        Self {
            current: Default::default(),
            iters: Default::default(),
        }
    }
}

impl<T> Iterator for SplitVecIntoSeqIter2<T>
where
    T: Send + Sync,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_next = self.current.next();
        match current_next.is_some() {
            true => current_next,
            false => match self.iters.pop() {
                Some(next) => {
                    self.current = next;
                    self.next()
                }
                None => None,
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining();
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for SplitVecIntoSeqIter2<T>
where
    T: Send + Sync,
{
    fn len(&self) -> usize {
        self.remaining()
    }
}

impl<T> FusedIterator for SplitVecIntoSeqIter2<T> where T: Send + Sync {}
