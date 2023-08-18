use crate::{fragment::fragment_struct::Fragment, FragmentGrowth, SplitVec};

impl<T> Default for SplitVec<T> {
    fn default() -> Self {
        let growth = FragmentGrowth::default();
        let capacity = growth.get_capacity(0);
        let fragment = Fragment::new(capacity);
        let fragments = vec![fragment];
        Self { fragments, growth }
    }
}
