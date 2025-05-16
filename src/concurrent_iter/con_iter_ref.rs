use crate::{Fragment, ParGrowth, SplitVec};
use orx_concurrent_iter::{
    IntoConcurrentIter,
    implementations::jagged_arrays::{
        AsRawSlice, AsSlice, ConIterJaggedRef, RawJaggedRef, RawSlice,
    },
};

impl<T> AsSlice<T> for Fragment<T> {
    fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<T> AsRawSlice<T> for Fragment<T> {
    fn ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    fn length(&self) -> usize {
        self.data.len()
    }

    fn raw_slice(&self, begin: usize, len: usize) -> RawSlice<T> {
        let end = begin + len;
        (&self.data[begin..end]).into()
    }
}

impl<'a, T, G> IntoConcurrentIter for &'a SplitVec<T, G>
where
    T: Send + Sync,
    G: ParGrowth,
{
    type Item = &'a T;

    type IntoIter = ConIterJaggedRef<'a, T, Fragment<T>, G>;

    fn into_con_iter(self) -> Self::IntoIter {
        let jagged = RawJaggedRef::new(&self.fragments, self.growth.clone(), Some(self.len));
        jagged.into_con_iter()
    }
}
