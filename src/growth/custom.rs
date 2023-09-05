use super::{
    any,
    growth_trait::{SplitVecGrowth, SplitVecGrowthWithFlexibleIndexAccess},
};
use crate::{Fragment, SplitVec};
use std::{fmt::Debug, rc::Rc};

pub(crate) type GetCapacityOfNewFragment<T> = dyn Fn(&[Fragment<T>]) -> usize;

/// Stategy which allows to define a custom growth rate with a function
/// of priorly created fragments.
///
/// # Examples
/// ```
/// use orx_split_vec::prelude::*;
/// use std::rc::Rc;
///
/// // vec: SplitVec<usize, CustomGrowth<usize>>
/// let mut vec =
///     SplitVec::with_custom_growth_function(Rc::new(|fragments: &[Fragment<_>]| {
///         if fragments.len() % 2 == 0 {
///             2
///         } else {
///             8
///         }
///     }));
///
///     for i in 0..100 {
///         vec.push(i);
///     }
///
///     vec.iter().zip(0..100).all(|(l, r)| *l == r);
///     
///     for (f, fragment) in vec.fragments().iter().enumerate() {
///         if f % 2 == 0 {
///             assert_eq!(2, fragment.capacity());
///         } else {
///             assert_eq!(8, fragment.capacity());
///         }
///     }
/// ```
pub struct CustomGrowth<T> {
    get_capacity_of_new_fragment: Rc<GetCapacityOfNewFragment<T>>,
}
impl<T> Clone for CustomGrowth<T> {
    fn clone(&self) -> Self {
        Self {
            get_capacity_of_new_fragment: self.get_capacity_of_new_fragment.clone(),
        }
    }
}

impl<T> Debug for CustomGrowth<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomGrowth").finish()
    }
}
impl<T> CustomGrowth<T> {
    /// Creates a new custom growth where the growth strategy is
    /// defined by the function `get_capacity_of_new_fragment`.
    ///
    /// # Panics
    /// Panics if the function returns 0 as the capacity of the new fragment
    /// for any given set of already created `fragments`.
    pub fn new(get_capacity_of_new_fragment: Rc<GetCapacityOfNewFragment<T>>) -> Self {
        Self {
            get_capacity_of_new_fragment,
        }
    }
}

impl<T: 'static> Default for CustomGrowth<T> {
    /// Creates the default custom growth which doubles the initial capacity every 4 fragments.
    ///
    /// Let c be the initial capacity of the split vector; i.e., capacity of the first fragment.
    /// Then, fragment capacities with the default function will be computed as:
    ///
    /// * capacities of fragments 0..4 = c
    /// * capacities of fragments 4..8 = 2c
    /// * capacities of fragments 8..12 = 3c
    /// * ...
    fn default() -> Self {
        fn default_exponential_growth<T>(fragments: &[Fragment<T>]) -> usize {
            let c = fragments.first().map(|f| f.capacity()).unwrap_or(4);
            let f = fragments.len();
            let m = (f / 4) as u32;
            c * usize::pow(2, m)
        }
        Self {
            get_capacity_of_new_fragment: Rc::new(default_exponential_growth::<T>),
        }
    }
}

impl<T> SplitVecGrowth<T> for CustomGrowth<T> {
    fn new_fragment_capacity(&self, fragments: &[Fragment<T>]) -> usize {
        let capacity = (self.get_capacity_of_new_fragment)(fragments);
        assert!(capacity > 0);
        capacity
    }
    fn get_fragment_and_inner_indices(
        &self,
        fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)> {
        any::get_fragment_and_inner_indices(fragments, element_index)
    }
}
impl<T> SplitVecGrowthWithFlexibleIndexAccess<T> for CustomGrowth<T> {}

impl<T> SplitVec<T, CustomGrowth<T>> {
    /// Creates a split vector with the custom grwoth strategy
    /// defined by the function `get_capacity_of_new_fragment`.
    ///
    /// # Examples
    /// ```
    /// use orx_split_vec::prelude::*;
    /// use std::rc::Rc;
    ///
    /// // vec: SplitVec<usize, CustomGrowth<usize>>
    /// let mut vec =
    ///     SplitVec::with_custom_growth_function(Rc::new(|fragments: &[Fragment<_>]| {
    ///         if fragments.len() % 2 == 0 {
    ///             2
    ///         } else {
    ///             8
    ///         }
    ///     }));
    ///
    ///     for i in 0..100 {
    ///         vec.push(i);
    ///     }
    ///
    ///     vec.iter().zip(0..100).all(|(l, r)| *l == r);
    ///     
    ///     for (f, fragment) in vec.fragments().iter().enumerate() {
    ///         if f % 2 == 0 {
    ///             assert_eq!(2, fragment.capacity());
    ///         } else {
    ///             assert_eq!(8, fragment.capacity());
    ///         }
    ///     }
    /// ```
    pub fn with_custom_growth_function(
        get_capacity_of_new_fragment: Rc<GetCapacityOfNewFragment<T>>,
    ) -> Self {
        let growth = CustomGrowth::new(get_capacity_of_new_fragment);
        Self {
            fragments: vec![],
            growth,
        }
    }
}
