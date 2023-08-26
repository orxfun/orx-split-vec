use crate::{SplitVec, SplitVecGrowth};
use orx_pinned_vec::PinnedVec;
use std::fmt::Debug;

impl<T, G> Debug for SplitVec<T, G>
where
    T: Debug,
    G: SplitVecGrowth<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as PinnedVec<T>>::debug(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn debug() {
        let mut vec = SplitVec::with_doubling_growth(1);
        for i in 0..8 {
            vec.push(i);
        }

        let debug_str = format!("{:?}", vec);
        assert_eq!("SplitVec [\n[0]\n[1, 2]\n[3, 4, 5, 6]\n[7]\n]\n", debug_str);
    }
}
