use crate::{SplitVec, SplitVecGrowth};
use orx_pinned_vec::PinnedVec;

impl<T, G, U> PartialEq<U> for SplitVec<T, G>
where
    U: AsRef<[T]>,
    T: PartialEq,
    G: SplitVecGrowth,
{
    fn eq(&self, other: &U) -> bool {
        <Self as PinnedVec<T>>::partial_eq(self, other.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;

    #[test]
    fn eq() {
        fn test<G: SplitVecGrowth>(mut vec: SplitVec<usize, G>) {
            for i in 0..142 {
                vec.push(i);
            }

            let eq_vec: Vec<_> = (0..vec.capacity()).collect();
            assert_eq!(vec, eq_vec);
        }
        test_all_growth_types!(test);
    }
}
