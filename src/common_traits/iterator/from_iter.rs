use crate::{Growth, SplitVec};
use orx_pinned_vec::PinnedVec;

impl<T, G: Growth> FromIterator<T> for SplitVec<T, G>
where
    SplitVec<T, G>: Default,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Self::default();
        for i in iter {
            vec.push(i)
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::{Doubling, Linear, SplitVec};

    #[test]
    fn collect() {
        let vec = SplitVec::<_, Doubling>::from_iter([0, 1, 2, 3, 4, 5].into_iter());
        assert_eq!(&vec, &[0, 1, 2, 3, 4, 5]);

        let vec = SplitVec::<_, Linear>::from_iter([0, 1, 2, 3, 4, 5].into_iter());
        assert_eq!(&vec, &[0, 1, 2, 3, 4, 5]);

        let vec = SplitVec::<_>::from_iter([0, 1, 2, 3, 4, 5].into_iter());
        assert_eq!(&vec, &[0, 1, 2, 3, 4, 5]);

        let vec: SplitVec<_, Doubling> = (0..6).collect();
        assert_eq!(&vec, &[0, 1, 2, 3, 4, 5]);

        let vec: SplitVec<_, Linear> = (0..6).collect();
        assert_eq!(&vec, &[0, 1, 2, 3, 4, 5]);

        let vec: SplitVec<_> = (0..6).collect();
        assert_eq!(&vec, &[0, 1, 2, 3, 4, 5]);

        let vec: SplitVec<_> = (0..6).filter(|x| x % 2 == 0).collect();
        assert_eq!(&vec, &[0, 2, 4]);

        let vec: SplitVec<_, Doubling> = (0..6).filter(|x| x % 2 == 0).collect();
        assert_eq!(&vec, &[0, 2, 4]);

        let vec: SplitVec<_, Linear> = (0..6).filter(|x| x % 2 == 0).collect();
        assert_eq!(&vec, &[0, 2, 4]);
    }
}
