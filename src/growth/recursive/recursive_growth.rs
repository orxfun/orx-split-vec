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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
