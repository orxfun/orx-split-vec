use crate::{ConcurrentSplitVec, GrowthWithConstantTimeAccess, SplitVec};
use orx_pinned_vec::IntoConcurrentPinnedVec;

impl<T, G: GrowthWithConstantTimeAccess> IntoConcurrentPinnedVec<T> for SplitVec<T, G> {
    type ConPinnedVec = ConcurrentSplitVec<T, G>;

    fn into_concurrent(self) -> Self::ConPinnedVec {
        self.into()
    }

    fn into_concurrent_filled_with<F>(mut self, fill_with: F) -> Self::ConPinnedVec
    where
        F: Fn() -> T,
    {
        if let Some(fragment) = self.fragments.last_mut() {
            let (len, capacity) = (fragment.len(), fragment.capacity());
            for _ in len..capacity {
                fragment.push(fill_with());
            }
        }

        self.into()
    }
}
