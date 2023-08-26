use super::iterator::SplitVecIterator;
use crate::{SplitVec, SplitVecGrowth};

impl<'a, T, G> IntoIterator for &'a SplitVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    type Item = &'a T;
    type IntoIter = SplitVecIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}
