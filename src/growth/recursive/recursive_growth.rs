use crate::{Doubling, Fragment, Growth, SplitVec};
use alloc::string::String;
use orx_pseudo_default::PseudoDefault;

/// Equivalent to [`Doubling`] strategy except for the following:
///
/// * enables zero-cost (no-ops) `append` operation:
///   * we can append standard vectors, vectors of vectors, split vectors, etc., any data that implements `IntoFragments` trait,
///   * by simply accepting it as a whole fragment,
///   * according to benchmarks documented in the crate definition:
///     * `SplitVec<_, Recursive>` is infinitely faster than other growth strategies or standard vector :)
///     * since its time complexity is independent of size of the data to be appended.
/// * at the expense of providing slower random-access performance:
///   * random access time complexity of `Doubling` strategy is constant time;
///   * that of `Recursive` strategy is linear in the number of fragments;
///   * according to benchmarks documented in the crate definition:
///     * `SplitVec<_, Doubling>` or standard vector are around 4 to 7 times faster than `SplitVec<_, Recursive>`,
///     * and 1.5 times faster when the elements get very large (16 x `u64`).
///
/// Note that other operations such as serial access are equivalent to `Doubling` strategy.
///
/// # Examples
///
/// ```
/// use orx_split_vec::*;
///
/// // SplitVec<usize, Recursive>
/// let mut vec = SplitVec::with_recursive_growth();
///
/// vec.push('a');
/// assert_eq!(vec, &['a']);
///
/// vec.append(vec!['b', 'c']);
/// assert_eq!(vec, &['a', 'b', 'c']);
///
/// vec.append(vec![vec!['d'], vec!['e', 'f']]);
/// assert_eq!(vec, &['a', 'b', 'c', 'd', 'e', 'f']);
///
/// let other_split_vec: SplitVec<_> = vec!['g', 'h'].into();
/// vec.append(other_split_vec);
/// assert_eq!(vec, &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Recursive;

impl PseudoDefault for Recursive {
    fn pseudo_default() -> Self {
        Default::default()
    }
}

impl Growth for Recursive {
    #[inline(always)]
    fn new_fragment_capacity_from(
        &self,
        fragment_capacities: impl ExactSizeIterator<Item = usize>,
    ) -> usize {
        Doubling.new_fragment_capacity_from(fragment_capacities)
    }

    fn maximum_concurrent_capacity<T>(
        &self,
        fragments: &[Fragment<T>],
        fragments_capacity: usize,
    ) -> usize {
        assert!(fragments_capacity >= fragments.len());

        let current_capacity = fragments.iter().map(|x| x.capacity()).sum();
        let mut last_capacity = fragments.last().map(|x| x.capacity()).unwrap_or(2);

        let mut total_capacity = current_capacity;

        for _ in fragments.len()..fragments_capacity {
            last_capacity *= 2;
            total_capacity += last_capacity;
        }

        total_capacity
    }

    fn required_fragments_len<T>(
        &self,
        fragments: &[Fragment<T>],
        maximum_capacity: usize,
    ) -> Result<usize, String> {
        fn overflown_err() -> String {
            alloc::format!(
                "Maximum cumulative capacity that can be reached by the Recursive strategy is {}.",
                usize::MAX
            )
        }

        let current_capacity: usize = fragments.iter().map(|x| x.capacity()).sum();
        let mut last_capacity = fragments.last().map(|x| x.capacity()).unwrap_or(2);

        let mut total_capacity = current_capacity;
        let mut f = fragments.len();

        while total_capacity < maximum_capacity {
            let (new_last_capacity, overflown) = last_capacity.overflowing_mul(2);
            if overflown {
                return Err(overflown_err());
            }
            last_capacity = new_last_capacity;

            let (new_total_capacity, overflown) = total_capacity.overflowing_add(last_capacity);
            if overflown {
                return Err(overflown_err());
            }

            total_capacity = new_total_capacity;
            f += 1;
        }

        Ok(f)
    }

    fn maximum_concurrent_capacity_bound<T>(
        &self,
        fragments: &[Fragment<T>],
        fragments_capacity: usize,
    ) -> usize {
        Doubling.maximum_concurrent_capacity_bound(fragments, fragments_capacity)
    }
}

impl<T> SplitVec<T, Recursive> {
    /// Strategy which allows to create a fragment with double the capacity
    /// of the prior fragment every time the split vector needs to expand.
    ///
    /// Notice that this is similar to the `Doubling` growth strategy.
    /// However, `Recursive` and `Doubling` strategies have the two following important differences in terms of performance:
    ///
    /// * Random access by indices is much faster with `Doubling`.
    /// * Recursive strategy enables copy-free `append` method which merges another vector to this vector in constant time.
    ///
    /// All other operations are expected to have similar complexity.
    ///
    /// ## Random Access
    ///
    /// * `Doubling` strategy provides a constant time access by random indices.
    /// * `Recursive` strategy provides a random access time complexity that is linear in the number of fragments.
    ///   Note that this is significantly faster than the linear-in-number-of-elements complexity of linked lists;
    ///   however, significantly slower than the `Doubling` strategy's constant time.
    ///
    /// ## Append
    ///
    /// * `Recursive` strategy provides `append` operation which allows merging two vectors in constant time without copies.
    ///
    /// `SplitVec::append` method should not be confused with `std::vec::Vec::append` method:
    /// * The split vector version consumes the vector to be appended.
    ///   It takes advantage of its split nature and appends the other vector simply by owning its pointer.
    ///   In other words, the other vector is appended to this vector with no cost and no copies.
    /// * The standard vector version mutates the vector to be appended,
    ///   moving all its element to the first vector leaving the latter empty.
    ///   This operation is carried out by memory copies.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// // SplitVec<usize, Doubling>
    /// let mut vec = SplitVec::with_recursive_growth();
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(4), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // fill the first 5 fragments
    /// let expected_fragment_capacities = vec![4, 8, 16, 32];
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
    pub fn with_recursive_growth() -> Self {
        SplitVec::with_doubling_growth().into()
    }

    /// Creates a new split vector with `Recursive` growth and initial `fragments_capacity`.
    ///
    /// This method differs from [`SplitVec::with_recursive_growth`] only by the pre-allocation of fragments collection.
    /// Note that this (only) important for concurrent programs:
    /// * SplitVec already keeps all elements pinned to their locations;
    /// * Creating a buffer for storing the meta information is important for keeping the meta information pinned as well.
    ///   This is relevant and important for concurrent programs.
    ///
    /// # Panics
    ///
    /// Panics if `fragments_capacity == 0`.
    pub fn with_recursive_growth_and_fragments_capacity(fragments_capacity: usize) -> Self {
        SplitVec::with_doubling_growth_and_fragments_capacity(fragments_capacity).into()
    }

    /// Creates a new split vector with `Recursive` growth and maximum concurrent capacity which depends
    /// on the pointer size of the target architecture.
    ///
    /// This method differs from [`SplitVec::with_recursive_growth`] only by the pre-allocation of fragments collection,
    /// which never contains more elements than 33.
    pub fn with_recursive_growth_and_max_concurrent_capacity() -> Self {
        SplitVec::with_doubling_growth_and_max_concurrent_capacity().into()
    }
}
