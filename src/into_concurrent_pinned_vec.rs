use crate::{ConcurrentSplitVec, GrowthWithConstantTimeAccess, SplitVec};
use orx_pinned_vec::IntoConcurrentPinnedVec;

impl<T, G: GrowthWithConstantTimeAccess> IntoConcurrentPinnedVec<T> for SplitVec<T, G> {
    type ConPinnedVec = ConcurrentSplitVec<T, G>;

    fn into_concurrent(self) -> Self::ConPinnedVec {
        self.into()
    }
}
