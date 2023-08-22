use crate::{Fragment, SplitVec, SplitVecGrowth};

impl<T, G> SplitVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    /// Creates an empty split vector with the given `growth` strategy.
    pub fn with_growth(growth: G) -> Self {
        let capacity = SplitVecGrowth::<T>::new_fragment_capacity(&growth, &[]);
        let fragment = Fragment::new(capacity);
        let fragments = vec![fragment];
        Self { fragments, growth }
    }
}
