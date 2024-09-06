use crate::{Growth, SplitVec};
use alloc::vec::Vec;
use orx_pinned_vec::PinnedVec;

// std::vec::vec
impl<T, G> From<SplitVec<T, G>> for Vec<T>
where
    G: Growth,
{
    /// Converts the `SplitVec` into a standard `Vec` with a contagious memory layout.
    ///
    /// If the split vector is composed of only one fragment, it is immediately returned as a `Vec` without any cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(2);
    /// split_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, split_vec.fragments().len());
    ///
    /// let vec: Vec<_> = split_vec.into();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(2);
    /// for i in 0..10 {
    ///     split_vec.push(i);
    /// }
    /// assert_eq!(&[0, 1, 2, 3], split_vec.fragments()[0].as_slice());
    /// assert_eq!(&[4, 5, 6, 7], split_vec.fragments()[1].as_slice());
    /// assert_eq!(&[8, 9], split_vec.fragments()[2].as_slice());
    ///
    /// let vec: Vec<_> = split_vec.into();
    /// assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec.as_slice());
    /// ```
    fn from(mut value: SplitVec<T, G>) -> Self {
        if value.fragments().len() == 1 {
            value
                .fragments
                .into_iter()
                .map(|x| x.data)
                .next()
                .expect("There exists exactly one fragment")
        } else {
            let mut vec = Vec::with_capacity(value.len());
            vec.reserve(value.len());
            for f in &mut value.fragments {
                vec.append(&mut f.data);
            }
            vec
        }
    }
}

impl<T, G> SplitVec<T, G>
where
    G: Growth,
{
    /// Converts the `SplitVec` into a standard `Vec` with a contagious memory layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(2);
    /// split_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, split_vec.fragments().len());
    ///
    /// let vec = split_vec.to_vec();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(2);
    /// for i in 0..10 {
    ///     split_vec.push(i);
    /// }
    /// assert_eq!(&[0, 1, 2, 3], split_vec.fragments()[0].as_slice());
    /// assert_eq!(&[4, 5, 6, 7], split_vec.fragments()[1].as_slice());
    /// assert_eq!(&[8, 9], split_vec.fragments()[2].as_slice());
    ///
    /// let vec = split_vec.to_vec();
    /// assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec.as_slice());
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use alloc::vec::Vec;

    #[test]
    fn single_fragment() {
        let mut split_vec = SplitVec::with_linear_growth(2);
        split_vec.extend_from_slice(&['a', 'b', 'c']);

        assert_eq!(1, split_vec.fragments().len());
        let vec: Vec<_> = split_vec.into();
        assert_eq!(vec, &['a', 'b', 'c']);

        let mut split_vec = SplitVec::with_doubling_growth();
        split_vec.extend_from_slice(&['a', 'b', 'c']);

        assert_eq!(1, split_vec.fragments().len());
        let vec: Vec<_> = split_vec.into();
        assert_eq!(vec, &['a', 'b', 'c']);

        let vec: Vec<_> = (0..1574).collect();
        let split_vec: SplitVec<_, Recursive> = vec.into();

        assert_eq!(1, split_vec.fragments().len());
        let vec: Vec<_> = split_vec.into();
        assert_eq!(1574, vec.len());
        for (i, val) in vec.iter().enumerate() {
            assert_eq!(i, *val);
        }
    }
}
