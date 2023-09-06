use crate::{fragment::fragment_struct::Fragment, Growth, SplitVec};

impl<T, G> Default for SplitVec<T, G>
where
    G: Growth + Default,
{
    /// Creates an empty split vector with the default `FragmentGrowth` strategy.
    fn default() -> Self {
        let growth = G::default();
        let capacity = Growth::new_fragment_capacity::<T>(&growth, &[]);
        let fragment = Fragment::new(capacity);
        let fragments = vec![fragment];
        Self { fragments, growth }
    }
}
