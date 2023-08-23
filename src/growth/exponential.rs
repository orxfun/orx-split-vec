use super::{
    any,
    growth_trait::{SplitVecGrowth, SplitVecGrowthWithFlexibleIndexAccess},
};
use crate::{Fragment, SplitVec};

/// Stategy which allows new fragments grow exponentially.
///
/// The capacity of the n-th fragment is computed as
/// `cap0 * pow(growth_coefficient, n)`
/// where `cap0` is the capacity of the first fragment.
///
/// Note that `DoublingGrowth` is a special case of `ExponentialGrowth`
/// with `growth_coefficient` equal to 2,
/// while providing a faster access by index.
///
/// On the other hand, exponential growth allows for fitting growth strategies
/// for fitting situations which could be a better choice when memory allocation
/// is more important than index access complexity.
///
/// As you may see in the example below, it is especially useful in providing
/// exponential growth rates slower than the doubling.
///
/// Assuming it is the common case compared to empty vector scenarios,
/// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
///
/// # Examples
///
/// ```
/// use orx_split_vec::SplitVec;
///
/// // SplitVec<usize, ExponentialGrowth>
/// let mut vec = SplitVec::with_exponential_growth(2, 1.5);
///
/// assert_eq!(1, vec.fragments().len());
/// assert_eq!(Some(2), vec.fragments().first().map(|f| f.capacity()));
/// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
///
/// // fill the first 5 fragments
/// let expected_fragment_capacities = vec![2, 3, 4, 6, 9, 13];
/// let num_items: usize = expected_fragment_capacities.iter().sum();
/// for i in 0..num_items {
///     vec.push(i);
/// }
///
/// assert_eq!(
///     expected_fragment_capacities,
///     vec.fragments()
///     .iter()
///     .map(|f| f.capacity())
///     .collect::<Vec<_>>()
/// );
/// assert_eq!(
///     expected_fragment_capacities,
///     vec.fragments().iter().map(|f| f.len()).collect::<Vec<_>>()
/// );
///
/// // create the 6-th fragment doubling the capacity
/// vec.push(42);
/// assert_eq!(
///     vec.fragments().len(),
///     expected_fragment_capacities.len() + 1
/// );
///
/// assert_eq!(vec.fragments().last().map(|f| f.capacity()), Some((13 as f32 * 1.5) as usize));
/// assert_eq!(vec.fragments().last().map(|f| f.len()), Some(1));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ExponentialGrowth {
    growth_coefficient: f32,
}
impl ExponentialGrowth {
    /// Creates a new exponential growth strategy with the given `growth_coefficient`.
    ///
    /// The capacity of the n-th fragment is computed as
    /// `cap0 * pow(growth_coefficient, n)`
    /// where `cap0` is the capacity of the first fragment.
    ///
    /// # Panics
    /// Panics if  `growth_coefficient` is less than 1.0.
    pub fn new(growth_coefficient: f32) -> Self {
        assert!(
            growth_coefficient >= 1.0,
            "Growth coefficient of exponential growth strategy must be greater than or equal to 1."
        );
        Self { growth_coefficient }
    }
    /// Returns the coefficient of the exponential growth strategy.
    pub fn growth_coefficient(&self) -> f32 {
        self.growth_coefficient
    }
}
impl Default for ExponentialGrowth {
    /// Creates a default exponential growth strategy with
    /// `growth_coefficient` being equal to 1.5.
    fn default() -> Self {
        Self {
            growth_coefficient: 1.5,
        }
    }
}

impl<T> SplitVecGrowth<T> for ExponentialGrowth {
    fn new_fragment_capacity(&self, fragments: &[Fragment<T>]) -> usize {
        fragments
            .last()
            .map(|f| (f.capacity() as f32 * self.growth_coefficient) as usize)
            .unwrap_or(4)
    }

    fn get_fragment_and_inner_indices(
        &self,
        fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)> {
        any::get_fragment_and_inner_indices(fragments, element_index)
    }
}
impl<T> SplitVecGrowthWithFlexibleIndexAccess<T> for ExponentialGrowth {}

impl<T> SplitVec<T, ExponentialGrowth> {
    /// Stategy which allows new fragments grow exponentially.
    ///
    /// The capacity of the n-th fragment is computed as
    /// `cap0 * pow(growth_coefficient, n)`
    /// where `cap0` is the capacity of the first fragment.
    ///
    /// Note that `DoublingGrowth` is a special case of `ExponentialGrowth`
    /// with `growth_coefficient` equal to 2,
    /// while providing a faster access by index.
    ///
    /// On the other hand, exponential growth allows for fitting growth startegies
    /// for fitting situations which could be a better choice when memory allocation
    /// is more important than index access complexity.
    ///
    /// As you may see in the example below, it is especially useful in providing
    /// exponential growth rates slower than the doubling.
    ///
    /// Assuming it is the common case compared to empty vector scenarios,
    /// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
    ///
    /// # Panics
    /// Panics if `first_fragment_capacity` is zero,
    /// or if `growth_coefficient` is less than 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// // SplitVec<usize, ExponentialGrowth>
    /// let mut vec = SplitVec::with_exponential_growth(2, 1.5);
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(2), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // fill the first 5 fragments
    /// let expected_fragment_capacities = vec![2, 3, 4, 6, 9, 13];
    /// let num_items: usize = expected_fragment_capacities.iter().sum();
    /// for i in 0..num_items {
    ///     vec.push(i);
    /// }
    ///
    /// assert_eq!(
    ///     expected_fragment_capacities,
    ///     vec.fragments()
    ///     .iter()
    ///     .map(|f| f.capacity())
    ///     .collect::<Vec<_>>()
    /// );
    /// assert_eq!(
    ///     expected_fragment_capacities,
    ///     vec.fragments().iter().map(|f| f.len()).collect::<Vec<_>>()
    /// );
    ///
    /// // create the 6-th fragment doubling the capacity
    /// vec.push(42);
    /// assert_eq!(
    ///     vec.fragments().len(),
    ///     expected_fragment_capacities.len() + 1
    /// );
    ///
    /// assert_eq!(vec.fragments().last().map(|f| f.capacity()), Some((13 as f32 * 1.5) as usize));
    /// assert_eq!(vec.fragments().last().map(|f| f.len()), Some(1));
    /// ```
    pub fn with_exponential_growth(
        first_fragment_capacity: usize,
        growth_coefficient: f32,
    ) -> Self {
        assert!(first_fragment_capacity > 0);
        assert!(growth_coefficient >= 1.0);
        Self {
            fragments: vec![Fragment::new(first_fragment_capacity)],
            growth: ExponentialGrowth::new(growth_coefficient),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ExponentialGrowth, Fragment, SplitVecGrowth};

    fn growth() -> ExponentialGrowth {
        ExponentialGrowth {
            growth_coefficient: 1.5,
        }
    }
    #[test]
    fn new_cap() {
        fn new_fra(cap: usize) -> Fragment<usize> {
            Vec::<usize>::with_capacity(cap).into()
        }

        let growth = growth();
        assert_eq!(3, growth.new_fragment_capacity(&[new_fra(2)]));
        assert_eq!(18, growth.new_fragment_capacity(&[new_fra(8), new_fra(12)]));
        assert_eq!(
            22,
            growth.new_fragment_capacity(&[new_fra(7), new_fra(10), new_fra(15)])
        );
    }

    #[test]
    fn indices_when_fragments_is_empty() {
        assert_eq!(
            None,
            <ExponentialGrowth as SplitVecGrowth<usize>>::get_fragment_and_inner_indices(
                &growth(),
                &[],
                0
            )
        );
    }

    #[test]
    fn indices() {
        fn new_full() -> Fragment<usize> {
            (0..10).collect::<Vec<_>>().into()
        }
        fn new_half() -> Fragment<usize> {
            let mut vec = Vec::with_capacity(15);
            for i in 0..3 {
                vec.push(10 + i);
            }
            vec.into()
        }

        let growth = growth();

        for i in 0..10 {
            assert_eq!(
                Some((0, i)),
                growth.get_fragment_and_inner_indices(&[new_full()], i)
            );
        }
        assert_eq!(
            None,
            growth.get_fragment_and_inner_indices(&[new_full()], 10)
        );

        for i in 0..10 {
            assert_eq!(
                Some((0, i)),
                growth.get_fragment_and_inner_indices(&[new_full(), new_half()], i)
            );
        }
        for i in 10..13 {
            assert_eq!(
                Some((1, i - 10)),
                growth.get_fragment_and_inner_indices(&[new_full(), new_half()], i)
            );
        }
        assert_eq!(
            None,
            growth.get_fragment_and_inner_indices(&[new_full(), new_half()], 13)
        );
    }
}
