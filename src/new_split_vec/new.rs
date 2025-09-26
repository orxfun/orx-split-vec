use crate::{Fragment, Growth, SplitVec};

impl<T> SplitVec<T> {
    /// Creates an empty split vector with default growth strategy.
    ///
    /// Default growth strategy is `Doubling` with initial capacity of 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let vec: SplitVec<f32> = SplitVec::new();
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(4, vec.fragments()[0].capacity());
    /// ```
    pub fn new() -> Self {
        Self::with_doubling_growth()
    }
}

impl<T, G> SplitVec<T, G>
where
    G: Growth,
{
    /// Creates an empty split vector with the given `growth` strategy.
    ///
    /// This constructor is especially useful to define custom growth strategies.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    /// use orx_pseudo_default::PseudoDefault;
    ///
    /// #[derive(Clone)]
    /// pub struct DoubleEverySecondFragment(usize); // any custom growth strategy
    ///
    /// impl PseudoDefault for DoubleEverySecondFragment {
    ///     fn pseudo_default() -> Self {
    ///         DoubleEverySecondFragment(1)
    ///     }
    /// }
    ///
    /// impl Growth for DoubleEverySecondFragment {
    ///     fn new_fragment_capacity_from(&self, fragment_capacities: impl ExactSizeIterator<Item = usize>) -> usize {
    ///         let num_fragments = fragment_capacities.len();
    ///         fragment_capacities
    ///             .last()
    ///             .map(|f| {
    ///                 let do_double = num_fragments % 2 == 0;
    ///                 if do_double {
    ///                     f * 2
    ///                 } else {
    ///                     f
    ///                 }
    ///             })
    ///             .unwrap_or(self.0)
    ///     }
    /// }
    /// let mut vec = SplitVec::with_growth(DoubleEverySecondFragment(8));
    /// for i in 0..17 {
    ///     vec.push(i);
    /// }
    ///
    /// assert_eq!(3, vec.fragments().len());
    ///
    /// assert_eq!(8, vec.fragments()[0].capacity());
    /// assert_eq!(8, vec.fragments()[0].len());
    ///
    /// assert_eq!(8, vec.fragments()[1].capacity());
    /// assert_eq!(8, vec.fragments()[1].len());
    ///
    /// assert_eq!(16, vec.fragments()[2].capacity());
    /// assert_eq!(1, vec.fragments()[2].len());
    /// ```
    pub fn with_growth(growth: G) -> Self {
        let capacity = Growth::new_fragment_capacity::<T>(&growth, &[]);
        let fragment = Fragment::new(capacity);
        let fragments = alloc::vec![fragment];
        SplitVec::from_raw_parts(0, fragments, growth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Doubling, Linear};

    #[test]
    fn new() {
        let vec: SplitVec<usize> = SplitVec::new();
        let vec: SplitVec<usize, Doubling> = vec;

        assert_eq!(1, vec.fragments().len());
        assert_eq!(4, vec.fragments()[0].capacity());
    }

    #[test]
    fn with_initial_capacity() {
        let vec: SplitVec<usize> = SplitVec::new();
        let vec: SplitVec<usize, Doubling> = vec;

        assert_eq!(1, vec.fragments().len());
        assert_eq!(4, vec.fragments()[0].capacity());
    }

    #[test]
    fn with_growth() {
        let vec: SplitVec<char, Linear> = SplitVec::with_growth(Linear::new(3));
        assert_eq!(1, vec.fragments().len());
        assert_eq!(8, vec.fragments()[0].capacity());

        let vec: SplitVec<char, Doubling> = SplitVec::with_growth(Doubling);
        assert_eq!(1, vec.fragments().len());
        assert_eq!(4, vec.fragments()[0].capacity());
    }
}
