//! # orx-split-vec
//!
//! A dynamic capacity vector with pinned elements.
//!
//! ## A. Motivation
//!
//! There might be various situations where pinned elements are helpful.
//!
//! * It is somehow required for async code, following [blog](https://blog.cloudflare.com/pin-and-unpin-in-rust) could be useful for the interested.
//! * It is a requirement to represent self-referential types with thin references.
//!
//! This crate focuses more on the latter. Particularly, it aims to make it safely and conveniently possible to build **self-referential collections** such as linked list, tree or graph.
//!
//! See [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) for complete documentation.
//!
//! `SplitVec` is one of the pinned vec implementations which can be wrapped by an [`ImpVec`](https://crates.io/crates/orx-imp-vec) and allow building self referential collections.
//!
//! ## B. Comparison with `FixedVec`
//!
//! [`FixedVec`](https://crates.io/crates/orx-fixed-vec) is another `PinnedVec` implementation aiming the same goal but with different features. You may see the comparison in the table below.
//!
//! | **`FixedVec`**                                                               | **`SplitVec`**                                                                   |
//! |------------------------------------------------------------------------------|----------------------------------------------------------------------------------|
//! | Implements `PinnedVec` => can be wrapped by an `ImpVec`.                     | Implements `PinnedVec` => can be wrapped by an `ImpVec`.                         |
//! | Requires exact capacity to be known while creating.                          | Can be created with any level of prior information about required capacity.      |
//! | Cannot grow beyond capacity; panics when `push` is called at capacity.       | Can grow dynamically. Further, it provides detailed control on how it must grow. |
//! | It is just a wrapper around `std::vec::Vec`; hence, has similar performance. | Performs additional tasks to provide flexibility; hence, slightly slower.        |
//!
//! ## C. Growth with Pinned Elements
//!
//! As the name suggests, `SplitVec` is a vector represented as a sequence of multiple contagious data fragments.
//!
//! The vector is at its capacity when all fragments are completely utilized. When the vector needs to grow further while at capacity, a new fragment is allocated. Therefore, growth does <ins>not</ins> require copying memory to a new memory location. Priorly pushed elements stay <ins>pinned</ins> to their memory locations.
//!
//! ### C.1. Available Growth Strategies
//!
//! The capacity of the new fragment is determined by the chosen growth strategy. Assume that `vec: SplitVec<_>` contains one fragment of capacity `C`, which is also the capacity of the vector since it is the only fragment. Assume, we used up all capacity; i.e., `vec.len() == vec.capacity()` (`C`). If we attempt to push a new element, `SplitVec` will allocate the second fragment with the following capacity:
//!
//! | **`Growth`** Strategy                   | 1st Fragment Capacity | 2nd Fragment Capacity | Vector Capacity |
//! |-----------------------------------------|-----------------------|-----------------------|-----------------|
//! | `Linear`                                | `C`                   | `C`                   | `2 * C`         |
//! | `Doubling`                              | `C`                   | `2 * C`               | `3 * C`         |
//! | `Exponential { growth_coefficient: a }` | `C`                   | `a * C`               | `(1 + a) * C`   |
//!
//! It is straightforward to derive the growth formula from the example. Further, you may notice that `Doubling` is a special case of `Exponential` where `growth_coefficient` is 2; the reason it co-exists is that it allows for faster element access in general.
//!
//! ### C.2. Custom Growth Strategies
//!
//! In order to define a custom growth strategy, one needs to implement the `Growth` trait. Implementation is straightforward. The trait contains two methods. The following method is required:
//!
//! ```rust ignore
//! fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize;
//! ```
//!
//! Notice that it takes as argument all priorly allocated fragments and needs to decide on the capacity of the new fragment.
//!
//! The second method `fn get_fragment_and_inner_indices<T>(&self, fragments: &[Fragment<T>], element_index: usize) -> Option<(usize, usize)>` has a default implementation and can be overwritten if the strategy allows for efficient computation of the indices.
//!
//! ## D. Examples
//!
//! ### D.1. Usage similar to `std::vec::Vec`
//!
//! ```rust
//! use orx_split_vec::prelude::*;
//!
//! let mut vec = SplitVec::new();
//!
//! vec.push(0);
//! vec.extend_from_slice(&[1, 2, 3]);
//! assert_eq!(vec, &[0, 1, 2, 3]);
//!
//! vec[0] = 10;
//! assert_eq!(10, vec[0]);
//!
//! vec.remove(0);
//! vec.insert(0, 0);
//!
//! assert_eq!(6, vec.iter().sum());
//!
//! assert_eq!(vec.clone(), vec);
//!
//! let stdvec: Vec<_> = vec.into();
//! assert_eq!(&stdvec, &[0, 1, 2, 3]);
//! ```
//!
//! ### D.2. `SplitVec` Specific Operations
//!
//! ```rust
//! use orx_split_vec::prelude::*;
//!
//! #[derive(Clone)]
//! struct MyCustomGrowth;
//! impl Growth for MyCustomGrowth {
//!     fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize {
//!         fragments.last().map(|f| f.capacity() + 1).unwrap_or(4)
//!     }
//! }
//!
//! // set the growth explicitly
//! let vec: SplitVec<i32, Linear> = SplitVec::with_linear_growth(4);
//! let vec: SplitVec<i32, Doubling> = SplitVec::with_doubling_growth();
//! let vec: SplitVec<i32, MyCustomGrowth> = SplitVec::with_growth(MyCustomGrowth);
//!
//! // methods revealing fragments
//! let mut vec = SplitVec::with_doubling_growth();
//! vec.extend_from_slice(&[0, 1, 2, 3]);
//!
//! assert_eq!(4, vec.capacity());
//! assert_eq!(1, vec.fragments().len());
//!
//! vec.push(4);
//! assert_eq!(vec, &[0, 1, 2, 3, 4]);
//!
//! assert_eq!(2, vec.fragments().len());
//! assert_eq!(4 + 8, vec.capacity());
//!
//! // SplitVec is not contagious; instead a collection of contagious fragments
//! // so it might or might not return a slice for a given range
//!
//! let slice: SplitVecSlice<_> = vec.try_get_slice(1..3);
//! assert_eq!(slice, SplitVecSlice::Ok(&[1, 2]));
//!
//! let slice = vec.try_get_slice(3..5);
//! // the slice spans from fragment 0 to fragment 1
//! assert_eq!(slice, SplitVecSlice::Fragmented(0, 1));
//!
//! let slice = vec.try_get_slice(3..7);
//! assert_eq!(slice, SplitVecSlice::OutOfBounds);
//!
//! // or the slice can be obtained as a vector of slices
//! let slice = vec.slice(0..3);
//! assert_eq!(1, slice.len());
//! assert_eq!(slice[0], &[0, 1, 2]);
//!
//! let slice = vec.slice(3..5);
//! assert_eq!(2, slice.len());
//! assert_eq!(slice[0], &[3]);
//! assert_eq!(slice[1], &[4]);
//!
//! let slice = vec.slice(0..vec.len());
//! assert_eq!(2, slice.len());
//! assert_eq!(slice[0], &[0, 1, 2, 3]);
//! assert_eq!(slice[1], &[4]);
//! ```
//!
//! ### D.3. Pinned Elements
//!
//! Unless elements are removed from the vector, the memory location of an element priorly pushed to the `SplitVec` <ins>never</ins> changes. This guarantee is utilized by `ImpVec` in enabling immutable growth to build self referential collections.
//!
//! ```rust
//! use orx_split_vec::prelude::*;
//!
//! let mut vec = SplitVec::with_linear_growth(3);
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
//! // let's push 80 new elements
//! for i in 1..81 {
//!     vec.push(i);
//! }
//!
//! for (i, elem) in vec.iter().enumerate() {
//!     assert_eq!(if i == 0 { 42 } else { i }, *elem);
//! }
//!
//! // now the split vector is composed of 11 fragments each with a capacity of 8 (2^3)
//! assert_eq!(11, vec.fragments().len());
//!
//! // the memory location of the first element remains intact
//! assert_eq!(addr42, &vec[0] as *const usize);
//!
//! // we can safely (using unsafe!) dereference it and read the correct value
//! assert_eq!(unsafe { *addr42 }, 42);
//! ```
//!
//! ## License
//!
//! This library is licensed under MIT license. See LICENSE for details.

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
pub use growth::{doubling::Doubling, growth_trait::Growth, linear::Linear};
pub use slice::SplitVecSlice;
pub use split_vec::SplitVec;

/// The split-vec prelude, along with the `SplitVec`, imports
/// various growth startegies, iterators and finally the `orx_pinned_vec::PinnedVec` trait.
pub mod prelude;
