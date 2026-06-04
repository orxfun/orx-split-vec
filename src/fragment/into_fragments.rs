use crate::{Fragment, Growth, SplitVec};
use alloc::vec::Vec;

/// Converts self into a collection of [`Fragment`]s.
pub trait IntoFragments<T> {
    /// Converts self into a collection of [`Fragment`]s.
    fn into_fragments(self) -> impl Iterator<Item = Fragment<T>>;
}

impl<T> IntoFragments<T> for Vec<T> {
    fn into_fragments(self) -> impl Iterator<Item = Fragment<T>> {
        let cap = Fragment::capacity_of_vec(&self);
        [Fragment::new_from_vec(self, cap)].into_iter()
    }
}

impl<T, const N: usize> IntoFragments<T> for [Vec<T>; N] {
    fn into_fragments(self) -> impl Iterator<Item = Fragment<T>> {
        self.into_iter().map(|x| {
            let cap = Fragment::capacity_of_vec(&x);
            Fragment::new_from_vec(x, cap)
        })
    }
}

impl<T> IntoFragments<T> for Vec<Vec<T>> {
    fn into_fragments(self) -> impl Iterator<Item = Fragment<T>> {
        self.into_iter().map(|x| {
            let cap = Fragment::capacity_of_vec(&x);
            Fragment::new_from_vec(x, cap)
        })
    }
}

impl<T, G: Growth> IntoFragments<T> for SplitVec<T, G> {
    fn into_fragments(self) -> impl Iterator<Item = Fragment<T>> {
        self.fragments.into_iter()
    }
}
