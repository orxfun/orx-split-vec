use crate::{Growth, SplitVec};
use alloc::vec::Vec;
use orx_pinned_vec::PinnedVec;

impl<T, G> Clone for SplitVec<T, G>
where
    T: Clone,
    G: Growth,
{
    fn clone(&self) -> Self {
        let mut fragments = Vec::with_capacity(self.fragments.capacity());

        for fragment in &self.fragments {
            let mut vec = Vec::with_capacity(fragment.capacity());
            vec.extend_from_slice(fragment);
            fragments.push(vec.into());
        }

        Self::from_raw_parts(self.len(), fragments, self.growth().clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn clone() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..57 {
                vec.push(i);
            }

            let clone = vec.clone();

            assert_eq!(vec.len(), clone.len());
            assert_eq!(vec.fragments().len(), clone.fragments().len());
            assert_eq!(vec.capacity(), clone.capacity());
            assert_eq!(vec.capacity_state(), clone.capacity_state());
            assert_eq!(
                vec.maximum_concurrent_capacity(),
                clone.maximum_concurrent_capacity()
            );

            for (a, b) in vec.fragments().iter().zip(clone.fragments().iter()) {
                assert_eq!(a.len(), b.len());
                assert_eq!(a.capacity(), b.capacity());

                for (x, y) in a.iter().zip(b.iter()) {
                    assert_eq!(x, y);
                }
            }
        }

        test_all_growth_types!(test);
    }
}
