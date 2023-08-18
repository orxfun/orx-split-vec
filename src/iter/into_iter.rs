use crate::SplitVec;

use super::iterator::SplitVecIterator;

impl<'a, T> IntoIterator for &'a SplitVec<T> {
    type Item = &'a T;
    type IntoIter = SplitVecIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}
