use crate::{Growth, SplitVec};

impl<T, G> Clone for SplitVec<T, G>
where
    T: Clone,
    G: Growth,
{
    fn clone(&self) -> Self {
        let fragments: Vec<_> = self
            .fragments
            .iter()
            .map(|fragment| {
                let mut vec = Vec::with_capacity(fragment.capacity());
                vec.extend_from_slice(fragment);
                vec.into()
            })
            .collect();
        Self {
            fragments,
            len: self.len,
            growth: self.growth.clone(),
        }
    }
}
