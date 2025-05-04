use super::con_iter::ConIterSplitVec;
use crate::{Growth, SplitVec};
use orx_concurrent_iter::IntoConcurrentIter;

impl<'a, T, G> IntoConcurrentIter for &'a SplitVec<T, G>
where
    T: Send + Sync,
    G: Growth,
{
    type Item = &'a T;

    type IntoIter = ConIterSplitVec<'a, T, G>;

    fn into_con_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}
