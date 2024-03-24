use crate::{Doubling, Fragment, Growth, SplitVec};

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

impl Growth for Recursive {
    #[inline(always)]
    fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize {
        Doubling.new_fragment_capacity(fragments)
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
    ) -> usize {
        let current_capacity: usize = fragments.iter().map(|x| x.capacity()).sum();
        let mut last_capacity = fragments.last().map(|x| x.capacity()).unwrap_or(2);

        let mut total_capacity = current_capacity;
        let mut f = fragments.len();

        while total_capacity < maximum_capacity {
            last_capacity *= 2;
            total_capacity += last_capacity;
            f += 1;
        }

        f
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
    /// Note that this is significantly faster than the linear-in-number-of-elements complexity of linked lists;
    /// however, significantly slower than the `Doubling` strategy's constant time.
    ///
    /// ## Append
    ///
    /// * `Recursive` strategy provides `append` operation which allows merging two vectors in constant time without copies.
    ///
    /// `SplitVec::append` method should not be confused with `std::vec::Vec::append` method:
    /// * The split vector version consumes the vector to be appended.
    /// It takes advantage of its split nature and appends the other vector simply by owning its pointer.
    /// In other words, the other vector is appended to this vector with no cost and no copies.
    /// * The standard vector version mutates the vector to be appended,
    /// moving all its element to the first vector leaving the latter empty.
    /// This operation is carried out by memory copies.
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
    /// This is relevant and important for concurrent programs.
    ///
    /// # Panics
    ///
    /// Panics if `fragments_capacity == 0`.
    pub fn with_recursive_growth_and_fragments_capacity(fragments_capacity: usize) -> Self {
        SplitVec::with_doubling_growth_and_fragments_capacity(fragments_capacity).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orx_pinned_vec::{PinnedVec, PinnedVecGrowthError};

    #[test]
    fn get_fragment_and_inner_indices() {
        let growth = Recursive;

        let vecs = vec![
            vec![0, 1, 2, 3],
            vec![4, 5],
            vec![6, 7, 8],
            vec![9],
            vec![10, 11, 12, 13, 14],
        ];
        let mut fragments: Vec<Fragment<_>> = vecs.clone().into_iter().map(|x| x.into()).collect();
        let len = fragments.iter().map(|x| x.len()).sum();

        let mut index = 0;
        for (f, vec) in vecs.iter().enumerate() {
            for (i, _) in vec.iter().enumerate() {
                let maybe_fi = growth.get_fragment_and_inner_indices(len, &fragments, index);
                assert_eq!(maybe_fi, Some((f, i)));

                let ptr = unsafe { growth.get_ptr_mut(&mut fragments, index) }.expect("is-some");
                assert_eq!(unsafe { *ptr }, index);

                unsafe { *ptr = 10 * index };
                assert_eq!(unsafe { *ptr }, 10 * index);

                index += 1;
            }
        }
    }

    #[test]
    fn get_fragment_and_inner_indices_exhaustive() {
        let growth = Recursive;

        let mut fragments: Vec<Fragment<_>> = vec![];

        let lengths = [30, 52, 14, 1, 7, 3, 79, 248, 147, 530];
        let mut index = 0;
        for _ in 0..100 {
            for &len in &lengths {
                let mut vec = Vec::with_capacity(len);
                for _ in 0..len {
                    vec.push(index);
                    index += 1;
                }
                fragments.push(vec.into());
            }
        }

        let total_len = fragments.iter().map(|x| x.len()).sum();

        let mut index = 0;
        let mut f = 0;
        for _ in 0..100 {
            for &len in &lengths {
                for i in 0..len {
                    let maybe_fi =
                        growth.get_fragment_and_inner_indices(total_len, &fragments, index);

                    assert_eq!(maybe_fi, Some((f, i)));

                    let ptr =
                        unsafe { growth.get_ptr_mut(&mut fragments, index) }.expect("is-some");
                    assert_eq!(unsafe { *ptr }, index);

                    unsafe { *ptr = 10 * index };
                    assert_eq!(unsafe { *ptr }, 10 * index);

                    index += 1;
                }
                f += 1;
            }
        }
    }

    #[test]
    fn maximum_concurrent_capacity() {
        fn max_cap<T>(vec: &SplitVec<T, Recursive>) -> usize {
            vec.growth()
                .maximum_concurrent_capacity(vec.fragments(), vec.fragments.capacity())
        }

        let mut vec: SplitVec<char, Recursive> = SplitVec::with_recursive_growth();
        assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32);

        let until = max_cap(&vec);
        for _ in 0..until {
            vec.push('x');
            assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32);
        }

        // fragments allocate beyond max_cap
        vec.push('x');
        assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32 + 64 + 128 + 256 + 512);
    }

    #[test]
    fn maximum_concurrent_capacity_when_appended() {
        fn max_cap<T>(vec: &SplitVec<T, Recursive>) -> usize {
            vec.growth()
                .maximum_concurrent_capacity(vec.fragments(), vec.fragments.capacity())
        }

        let mut vec: SplitVec<char, Recursive> = SplitVec::with_recursive_growth();
        assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32);

        vec.append(vec!['x'; 10]);

        assert_eq!(max_cap(&vec), 4 + 10 + 20 + 40);
    }

    #[test]
    fn with_recursive_growth() {
        let mut vec: SplitVec<char, _> = SplitVec::with_recursive_growth();

        assert_eq!(4, vec.fragments.capacity());

        for _ in 0..100_000 {
            vec.push('x');
        }

        assert!(vec.fragments.capacity() > 4);

        let mut vec: SplitVec<char, _> = SplitVec::with_recursive_growth();
        let result = unsafe { vec.grow_to(100_000) };
        assert!(result.is_ok());
        assert!(result.expect("is-ok") >= 100_000);
    }

    #[test]
    fn with_recursive_growth_and_fragments_capacity_normal_growth() {
        let mut vec: SplitVec<char, _> = SplitVec::with_recursive_growth_and_fragments_capacity(1);

        assert_eq!(1, vec.fragments.capacity());

        for _ in 0..100_000 {
            vec.push('x');
        }

        assert!(vec.fragments.capacity() > 4);
    }

    #[test]
    fn with_recursive_growth_and_fragments_capacity_concurrent_grow_never() {
        let mut vec: SplitVec<char, _> = SplitVec::with_recursive_growth_and_fragments_capacity(1);

        assert!(!vec.can_concurrently_add_fragment());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1) };
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );
    }

    #[test]
    fn with_recursive_growth_and_fragments_capacity_concurrent_grow_once() {
        let mut vec: SplitVec<char, _> = SplitVec::with_recursive_growth_and_fragments_capacity(2);

        assert!(vec.can_concurrently_add_fragment());

        let next_capacity = vec.capacity() + vec.growth().new_fragment_capacity(vec.fragments());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1) };
        assert_eq!(result, Ok(next_capacity));

        assert!(!vec.can_concurrently_add_fragment());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1) };
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );
    }

    #[test]
    fn with_recursive_growth_and_fragments_capacity_concurrent_grow_twice() {
        // when possible
        let mut vec: SplitVec<char, _> = SplitVec::with_recursive_growth_and_fragments_capacity(3);

        assert!(vec.can_concurrently_add_fragment());

        let fragment_2_capacity = vec.growth().new_fragment_capacity(vec.fragments());
        let fragment_3_capacity = fragment_2_capacity * 2;
        let new_capacity = vec.capacity() + fragment_2_capacity + fragment_3_capacity;

        let result = unsafe { vec.concurrently_grow_to(new_capacity - 1) };
        assert_eq!(result, Ok(new_capacity));

        assert!(!vec.can_concurrently_add_fragment());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1) };
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );

        // when not possible
        let mut vec: SplitVec<char, _> = SplitVec::with_recursive_growth_and_fragments_capacity(2);

        assert!(vec.can_concurrently_add_fragment()); // although we can add one fragment

        let result = unsafe { vec.concurrently_grow_to(new_capacity - 1) }; // we cannot add two
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );
    }

    #[test]
    #[should_panic]
    fn with_recursive_growth_and_fragments_capacity_zero() {
        let _: SplitVec<char, _> = SplitVec::with_recursive_growth_and_fragments_capacity(0);
    }

    #[test]
    fn required_fragments_len() {
        let vec: SplitVec<char, Recursive> = SplitVec::with_recursive_growth();
        let num_fragments = |max_cap| {
            vec.growth()
                .required_fragments_len(vec.fragments(), max_cap)
        };

        // 4 - 12 - 28 - 60 - 124
        assert_eq!(num_fragments(0), 1);
        assert_eq!(num_fragments(1), 1);
        assert_eq!(num_fragments(4), 1);
        assert_eq!(num_fragments(5), 2);
        assert_eq!(num_fragments(12), 2);
        assert_eq!(num_fragments(13), 3);
        assert_eq!(num_fragments(36), 4);
        assert_eq!(num_fragments(67), 5);
        assert_eq!(num_fragments(136), 6);
    }

    #[test]
    fn required_fragments_len_when_appended() {
        let mut vec: SplitVec<char, Recursive> = SplitVec::with_recursive_growth();
        vec.append(vec!['x'; 10]);

        let num_fragments = |max_cap| {
            vec.growth()
                .required_fragments_len(vec.fragments(), max_cap)
        };

        // 4 - 10 - 20 - 40 - 80
        // 4 - 14 - 34 - 74 - 154
        assert_eq!(num_fragments(0), 2);
        assert_eq!(num_fragments(1), 2);
        assert_eq!(num_fragments(14), 2);
        assert_eq!(num_fragments(15), 3);
        assert_eq!(num_fragments(21), 3);
        assert_eq!(num_fragments(34), 3);
        assert_eq!(num_fragments(35), 4);
        assert_eq!(num_fragments(74), 4);
        assert_eq!(num_fragments(75), 5);
        assert_eq!(num_fragments(154), 5);
        assert_eq!(num_fragments(155), 6);
    }
}
