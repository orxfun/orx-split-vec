use crate::{Growth, SplitVec};

impl<T, G> Default for SplitVec<T, G>
where
    G: Growth + Default,
{
    /// Creates an empty split vector with the default `FragmentGrowth` strategy.
    fn default() -> Self {
        Self::with_growth(G::default())
    }
}
