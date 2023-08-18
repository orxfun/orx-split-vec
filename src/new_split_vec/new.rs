use crate::{Fragment, FragmentGrowth, SplitVec};

impl<T> SplitVec<T> {
    /// Creates an empty split vector with the given `growth` strategy.
    pub fn with_growth(growth: FragmentGrowth) -> Self {
        let capacity = growth.get_capacity(0);
        let fragment = Fragment::new(capacity);
        let fragments = vec![fragment];
        Self { fragments, growth }
    }
}
