use crate::{Fragment, ParGrowth, SplitVec};
use orx_concurrent_iter::{
    IntoConcurrentIter,
    implementations::jagged_arrays::{ConIterJaggedRef, RawJaggedRef, Slices},
};

pub struct Fragments<'a, T>(&'a [Fragment<T>]);

impl<T> Clone for Fragments<'_, T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'a, T: 'a> Slices<'a, T> for Fragments<'a, T> {
    fn empty() -> Self {
        Fragments(Default::default())
    }

    fn num_slices(&self) -> usize {
        self.0.len()
    }

    fn slices(&self) -> impl Iterator<Item = &'a [T]> {
        self.0.iter().map(|x| x.as_slice())
    }

    fn lengths(&self) -> impl Iterator<Item = usize> {
        self.0.iter().map(|x| x.len())
    }

    fn slice_at(&self, f: usize) -> Option<&'a [T]> {
        self.0.get(f).map(|x| x.as_slice())
    }

    unsafe fn slice_at_unchecked(&self, f: usize) -> &'a [T] {
        self.0[f].as_slice()
    }
}

impl<'a, T, G> IntoConcurrentIter for &'a SplitVec<T, G>
where
    T: Send + Sync,
    G: ParGrowth,
{
    type Item = &'a T;

    type IntoIter = ConIterJaggedRef<'a, T, Fragments<'a, T>, G>;

    fn into_con_iter(self) -> Self::IntoIter {
        let jagged = RawJaggedRef::new(
            Fragments(&self.fragments),
            self.growth.clone(),
            Some(self.len),
        );
        jagged.into_con_iter()
    }
}
