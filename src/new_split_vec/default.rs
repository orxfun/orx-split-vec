use crate::{fragment::fragment_struct::Fragment, SplitVec, SplitVecGrowth};

impl<T, G> Default for SplitVec<T, G>
where
    G: SplitVecGrowth + Default,
{
    /// Creates an empty split vector with the default `FragmentGrowth` strategy.
    fn default() -> Self {
        let growth = G::default();
        let capacity = SplitVecGrowth::new_fragment_capacity::<T>(&growth, &[]);
        let fragment = Fragment::new(capacity);
        let fragments = vec![fragment];
        Self { fragments, growth }
    }
}
