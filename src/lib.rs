//! A split vector, `SplitVec`, is a vector represented as a sequence of
//! multiple contagious data fragments.
//!
//! It provides the following features:
//!
//! * Flexible in growth strategies; custom strategies can be defined.
//! * Growth does not cause any memory copies.
//! * Capacity of an already created fragment is never changed.
//! * The above feature allows the data to stay **pinned** in place.
//!     * `SplitVec<T>` implements [`PinnedVec<T>`](https://crates.io/crates/orx-pinned-vec) for any `T`;
//!     * `SplitVec<T>` implements `PinnedVecSimple<T>` for `T: NotSelfRefVecItem`;
//!     * Memory location of an item added to the split vector will never change
//! unless the vector is dropped or cleared.
//!     * This allows the split vec to be converted into an [`ImpVec`](https://crates.io/crates/orx-imp-vec)
//! to enable immutable-push operations which allows for
//! convenient, efficient and safe implementations of self-referencing data structures.
//!
//! ## Pinned elements
//!
//! ```rust
//! use orx_split_vec::prelude::*;
//!
//! let mut vec = SplitVec::with_linear_growth(10);
//!
//! // split vec with 1 item in 1 fragment
//! vec.push(42usize);
//! assert_eq!(&[42], &vec);
//! assert_eq!(1, vec.fragments().len());
//! assert_eq!(&[42], &vec.fragments()[0]);
//!
//! // let's get a pointer to the first element
//! let addr42 = &vec[0] as *const usize;
//!
//! // let's push 100 new elements
//! for i in 1..101 {
//!     vec.push(i);
//! }
//!
//! for (i, elem) in vec.iter().enumerate() {
//!     assert_eq!(if i == 0 { 42 } else { i }, *elem);
//! }
//!
//! // now the split vector is composed of 11 fragments each with a capacity of 10
//! assert_eq!(11, vec.fragments().len());
//!
//! // the memory location of the first element remains intact
//! assert_eq!(addr42, &vec[0] as *const usize);
//!
//! // we can safely (using unsafe!) dereference it and read the correct value
//! assert_eq!(unsafe { *addr42 }, 42);
//! ```
//!
//! ## Vector with self referencing elements
//!
//! `SplitVec` is not meant to be a replacement for `std::vec::Vec`,
//! and not preferable over it in most of the cases since it adds one level of abstraction.
//!
//! However, it is useful and convenient in defining data structures, child structures of which
//! hold references to each other.
//! This is a very common and useful property for trees, graphs, etc.
//! SplitVec allows to store children of such structures in a vector with the following features:
//!
//! * holding children close to each other allows for better cache locality,
//! * reduces heap allocations and utilizes **thin** references rather than wide pointers,
//! * while still guaranteeing that the references will remain valid.
//!
//! `SplitVec` receives this feature due to the following:
//!
//! * `SplitVec` implements `PinnedVec`; and hence, it can be wrapped by an `ImpVec`,
//! * `ImpVec` allows safely building the vector where items are referencing each other,
//! * `ImpVec` can then be converted back to the underlying `SplitVec`
//! having the abovementioned features and safety guarantees.
//!
//! ### Flexible growth strategies without copies
//!
//! In addition, `SplitVec` is useful for building collections when:
//!
//! * there is high uncertainty in the expected length, and
//! * copies are expensive.
//!
//! In this case, `SplitVec` provides a detailed control on how the memory should grow.
//! Further, it avoids copies while growing.
//! Instead, every time the vector needs to grow, it allocates a new chunk of memory
//! as a separate fragment.
//!
//!
//! ```rust
//! use orx_split_vec::prelude::*;
//! #[derive(Clone)]
//! pub struct DoubleEverySecondFragment(usize); // any custom growth strategy
//! impl Growth for DoubleEverySecondFragment {
//!     fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize {
//!         fragments
//!             .last()
//!             .map(|f| {
//!                 let do_double = fragments.len() % 2 == 0;
//!                 if do_double {
//!                     f.capacity() * 2
//!                 } else {
//!                     f.capacity()
//!                 }
//!             })
//!             .unwrap_or(self.0)
//!     }
//! }
//!
//! fn get_fragment_capacities<T, G: Growth>(vec: &SplitVec<T, G>) -> Vec<usize> {
//!     vec.fragments().iter().map(|f| f.capacity()).collect()
//! }
//! fn get_fragment_lengths<T, G: Growth>(vec: &SplitVec<T, G>) -> Vec<usize> {
//!     vec.fragments().iter().map(|f| f.len()).collect()
//! }
//!
//! // let's create 4 vectors with different growth strategies
//! let mut vec_lin = SplitVec::with_linear_growth(10);
//! let mut vec_dbl = SplitVec::with_doubling_growth(4);
//! let mut vec_exp = SplitVec::with_exponential_growth(4, 1.5);
//! let mut vec_custom = SplitVec::with_growth(DoubleEverySecondFragment(1));
//!
//! // and push 35 elements to all vectors
//! for i in 0..35 {
//!     vec_lin.push(i);
//!     vec_dbl.push(i);
//!     vec_exp.push(i);
//!     vec_custom.push(i);
//! }
//!
//! // # linear: fragments of equal capacities
//! assert_eq!(vec![10, 10, 10, 10], get_fragment_capacities(&vec_lin));
//! assert_eq!(vec![10, 10, 10, 5], get_fragment_lengths(&vec_lin));
//!
//! // # doubling: fragment capacities keep doubling
//! assert_eq!(vec![4, 8, 16, 32], get_fragment_capacities(&vec_dbl));
//! assert_eq!(vec![4, 8, 16, 7], get_fragment_lengths(&vec_dbl));
//!
//! // # exponential: fragment capacities grow exponentially with given growth factor
//! assert_eq!(vec![4, 6, 9, 13, 19], get_fragment_capacities(&vec_exp));
//! assert_eq!(vec![4, 6, 9, 13, 3], get_fragment_lengths(&vec_exp));
//!
//! // # custom: pretty much any growth strategy
//! assert_eq!(
//!     vec![1, 1, 2, 2, 4, 4, 8, 8, 16],
//!     get_fragment_capacities(&vec_custom)
//! );
//! assert_eq!(vec![1, 1, 2, 2, 4, 4, 8, 8, 5], get_fragment_lengths(&vec_custom));
//! ```

#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]

mod common_traits;
mod eq;
mod fragment;
mod growth;
mod new_split_vec;
mod pinned_vec;
mod resize_multiple;
mod slice;
mod split_vec;
#[cfg(test)]
pub(crate) mod test;

pub use common_traits::iterator::iter::Iter;
pub use fragment::fragment_struct::Fragment;
pub use growth::{
    doubling::Doubling, exponential::Exponential, growth_trait::Growth, linear::Linear,
};
pub use slice::SplitVecSlice;
pub use split_vec::SplitVec;

/// The split-vec prelude, along with the `SplitVec`, imports
/// various growth startegies, iterators and finally the `orx_pinned_vec::PinnedVec` trait.
pub mod prelude;
