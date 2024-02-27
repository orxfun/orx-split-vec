use crate::{eq::are_fragments_eq_to_slice, Growth, SplitVec};

impl<T, G, U> PartialEq<U> for SplitVec<T, G>
where
    U: AsRef<[T]>,
    T: PartialEq,
    G: Growth,
{
    fn eq(&self, other: &U) -> bool {
        are_fragments_eq_to_slice(&self.fragments, other.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;

    #[test]
    fn eq() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..142 {
                vec.push(i);
            }

            let eq_vec: Vec<_> = (0..vec.capacity()).collect();
            assert_eq!(vec, eq_vec);
        }
        test_all_growth_types!(test);
    }
}
