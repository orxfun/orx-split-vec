use super::growth_trait::SplitVecGrowth;
use crate::{Fragment, SplitVec};

/// Stategy which allows creates a fragment with double the capacity
/// of the prior fragment every time the split vector needs to expand.
///
/// Assuming it is the common case compared to empty vector scenarios,
/// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
///
/// # Examples
///
/// ```
/// use orx_split_vec::prelude::*;
///
/// // SplitVec<usize, DoublingGrowth>
/// let mut vec = SplitVec::with_doubling_growth(2);
///
/// assert_eq!(1, vec.fragments().len());
/// assert_eq!(Some(2), vec.fragments().first().map(|f| f.capacity()));
/// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
///
/// // fill the first 5 fragments
/// let expected_fragment_capacities = vec![2, 4, 8, 16, 32];
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
/// assert_eq!(vec.fragments().last().map(|f| f.capacity()), Some(32 * 2));
/// assert_eq!(vec.fragments().last().map(|f| f.len()), Some(1));
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Doubling;

impl SplitVecGrowth for Doubling {
    fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize {
        fragments.last().map(|f| f.capacity() * 2).unwrap_or(4)
    }

    fn get_fragment_and_inner_indices<T>(
        &self,
        fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)> {
        let c = fragments.first().map(|f| f.capacity()).unwrap_or(4);

        if element_index < c && element_index < fragments[0].len() {
            Some((0, element_index))
        } else {
            let f = ((element_index + c) as f32 / c as f32).log2() as usize;
            let beg = (usize::pow(2, f as u32) - 1) * c;
            let i = element_index - beg;
            if f < fragments.len() && i < fragments[f].len() {
                Some((f, i))
            } else {
                None
            }
        }
    }
}

impl<T> SplitVec<T, Doubling> {
    /// Stategy which allows to create a fragment with double the capacity
    /// of the prior fragment every time the split vector needs to expand.
    ///
    /// Assuming it is the common case compared to empty vector scenarios,
    /// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
    ///
    /// # Panics
    /// Panics if `first_fragment_capacity` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// // SplitVec<usize, DoublingGrowth>
    /// let mut vec = SplitVec::with_doubling_growth(2);
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(2), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // fill the first 5 fragments
    /// let expected_fragment_capacities = vec![2, 4, 8, 16, 32];
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
    /// assert_eq!(vec.fragments().last().map(|f| f.capacity()), Some(32 * 2));
    /// assert_eq!(vec.fragments().last().map(|f| f.len()), Some(1));
    /// ```
    pub fn with_doubling_growth(first_fragment_capacity: usize) -> Self {
        assert!(first_fragment_capacity > 0);
        Self {
            fragments: vec![Fragment::new(first_fragment_capacity)],
            growth: Doubling,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Doubling, Fragment, SplitVecGrowth};

    #[test]
    fn new_cap() {
        fn new_fra(cap: usize) -> Fragment<usize> {
            Vec::<usize>::with_capacity(cap).into()
        }

        let growth = Doubling;
        assert_eq!(4, growth.new_fragment_capacity(&[new_fra(2)]));
        assert_eq!(12, growth.new_fragment_capacity(&[new_fra(3), new_fra(6)]));
        assert_eq!(
            56,
            growth.new_fragment_capacity(&[new_fra(7), new_fra(14), new_fra(28)])
        );
    }

    #[test]
    #[should_panic]
    fn indices_panics_when_fragments_is_empty() {
        assert_eq!(
            None,
            <Doubling as SplitVecGrowth>::get_fragment_and_inner_indices::<usize>(
                &Doubling,
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
            let mut vec = Vec::with_capacity(20);
            for i in 0..5 {
                vec.push(10 + i);
            }
            vec.into()
        }

        let growth = Doubling;

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
        for i in 10..15 {
            assert_eq!(
                Some((1, i - 10)),
                growth.get_fragment_and_inner_indices(&[new_full(), new_half()], i)
            );
        }
        assert_eq!(
            None,
            growth.get_fragment_and_inner_indices(&[new_full(), new_half()], 15)
        );
    }
}
