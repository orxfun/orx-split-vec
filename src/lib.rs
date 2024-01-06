//! # orx-split-vec
//!
//! An efficient constant access time vector with dynamic capacity and pinned elements.
//!
//! ## A. Motivation
//!
//! There might be various situations where pinned elements are helpful.
//!
//! * It is somehow required for async code, following [blog](https://blog.cloudflare.com/pin-and-unpin-in-rust) could be useful for the interested.
//! * It is crucial in representing self-referential types with thin references.
//!
//! This crate focuses on the latter. Particularly, it aims to make it safe and convenient to build **performant self-referential collections** such as linked lists, trees or graphs. See [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) for complete documentation on the motivation.
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
//! | Cannot grow beyond capacity; panics when `push` is called at capacity.       | Can grow dynamically. Further, it provides control on how it must grow. |
//! | It is just a wrapper around `std::vec::Vec`; hence, has equivalent performance. | Performance-optimized built-in growth strategies also have `std::vec::Vec` equivalent performance. |
//!
//! After the performance optimizations on the `SplitVec`, it is now comparable to `std::vec::Vec` in terms of performance (see <a href="#section-benchmarks">E. Benchmarks</a> for the experiments). This might make `SplitVec` a dominating choice over `FixedVec`.
//!
//! ## C. Growth with Pinned Elements
//!
//! As the name suggests, `SplitVec` is a vector represented as a sequence of multiple contagious data fragments.
//!
//! The vector is said to be at its capacity when all fragments are completely utilized. When the vector needs to grow further while at capacity, a new fragment is allocated. Therefore, growth does <ins>not</ins> require copying memory to a new memory location. Priorly pushed elements stay <ins>pinned</ins> to their memory locations.
//!
//! ### C.1. Available Growth Strategies
//!
//! The capacity of the new fragment is determined by the chosen growth strategy. Assume that `vec: SplitVec<_, G>` where `G: Growth` contains one fragment of capacity `C`, which is also the capacity of the vector since it is the only fragment. Assume, we used up all capacity; i.e., `vec.len() == vec.capacity()` (`C`). If we attempt to push a new element, `SplitVec` will allocate the second fragment with the following capacity:
//!
//! | **`Growth`** Strategy                   | 1st Fragment Capacity | 2nd Fragment Capacity | Vector Capacity |
//! |-----------------------------------------|-----------------------|-----------------------|-----------------|
//! | `Linear`                                | `C`                   | `C`                   | `2 * C`         |
//! | `Doubling`                              | `C`                   | `2 * C`               | `3 * C`         |
//!
//! `C` is set on initialization as a power of two for `Linear` strategy, and it is fixed to 4 for `Doubling` strategy to allow for access time optimizations.
//!
//! ### C.2. Custom Growth Strategies
//!
//! In order to define a custom growth strategy, one needs to implement the `Growth` trait. Implementation is straightforward. The trait contains two methods. The following method is required:
//!
//! ```rust ignore
//! fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize
//! ```
//!
//! Notice that it takes as argument all priorly allocated fragments and needs to decide on the capacity of the new fragment.
//!
//! The second method `fn get_fragment_and_inner_indices<T>(&self, vec_len: usize, fragments: &[Fragment<T>], element_index: usize) -> Option<(usize, usize)>` has a default implementation and can be overwritten if the strategy allows for efficient computation of the indices.
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
//! let mut vec = SplitVec::new(); // Doubling growth as the default strategy
//!
//! // split vec with 1 item in 1 fragment
//! vec.push(42usize);
//!
//! assert_eq!(&[42], &vec);
//! assert_eq!(1, vec.fragments().len());
//! assert_eq!(&[42], &vec.fragments()[0]);
//!
//! // let's get a pointer to the first element
//! let addr42 = &vec[0] as *const usize;
//!
//! // let's push 3 + 8 + 16 new elements to end up with 3 fragments
//! for i in 1..(3 + 8 + 16) {
//!     vec.push(i);
//! }
//!
//! for (i, elem) in vec.iter().enumerate() {
//!     assert_eq!(if i == 0 { 42 } else { i }, *elem);
//! }
//!
//! // now the split vector is composed of 11 fragments each with a capacity of 10
//! assert_eq!(3, vec.fragments().len());
//!
//! // the memory location of the first element remains intact
//! assert_eq!(addr42, &vec[0] as *const usize);
//!
//! // we can safely dereference it and read the correct value
//! // the method is still unsafe for SplitVec
//! // but the undelrying guarantee will be used by ImpVec
//! assert_eq!(unsafe { *addr42 }, 42);
//! ```
//!
//! <div id="section-benchmarks"></div>
//!
//! ## E. Benchmarks
//!
//! Recall that the motivation of using a split vector is to get benefit of the pinned elements, rather than to be used in place of the standard vector which is highly efficient. The aim of the performance optimizations and benchmarks is to make sure that the gap is kept within acceptable and constant limits. `SplitVec` seems to comfortably satisfy this. After optimizations, built-in growth strategies appear to have a similar peformance to `std::vec::Vec` in growth, serial access and random access benchmarks.
//!
//! *You may find the details of each benchmark in the following subsections. All the numbers in tables below represent duration in milliseconds.*
//!
//! ### E.1. Grow
//!
//! *You may see the benchmark at [benches/grow.rs](https://github.com/orxfun/orx-split-vec/blob/main/benches/grow.rs).*
//!
//! The benchmark compares the build up time of vectors by pushing elements one by one. The baseline is the vector created by `std::vec::Vec::with_capacity` which has the perfect information on the number of elements to be pushed. The compared variants are vectors created with no prior knowledge about capacity: `std::vec::Vec::new`, `SplitVec<_, Linear>` and `SplitVec<_, Doubling>`.
//!
//! <img src="https://raw.githubusercontent.com/orxfun/orx-split-vec/main/docs/img/bench_grow.PNG" alt="https://raw.githubusercontent.com/orxfun/orx-split-vec/main/docs/img/bench_grow.PNG" />
//!
//! The baseline **std_vec_with_capacity** performs between 1.5 and 2.0 times faster than **std_vec_new** which has no capacity information and requires copies while growing. As mentioned before, **`SplitVec`** growth is copy-free guaranteeing that pushed elements stay pinned. Therefore, it is expected to perform in between. However, it performs almost as well as, and sometimes faster than, std_vec_with_capacity.
//!
//!
//! ### E.2. Random Access
//!
//! *You may see the benchmark at [benches/random_access.rs](https://github.com/orxfun/orx-split-vec/blob/main/benches/random_access.rs).*
//!
//! In this benchmark, we access vector elements by indices in a random order. Here the baseline is again the standard vector created by `Vec::with_capacity`, which is compared with `Linear` and `Doubling` growth strategies of the `SplitVec` which are optimized specifically for the random access.
//!
//! <img src="https://raw.githubusercontent.com/orxfun/orx-split-vec/main/docs/img/bench_random_access.PNG" alt="https://raw.githubusercontent.com/orxfun/orx-split-vec/main/docs/img/bench_random_access.PNG" />
//!
//! We can see that `Linear` is slower than `Doubling`. The difference of performances between `SplitVec<_, Doubling>` (the default growth) is always less than 50% and approaches to zero as the element size or number of elements gets larger.
//!
//!
//! ### E.3. Serial Access
//!
//! *You may see the benchmark at [benches/serial_access.rs](https://github.com/orxfun/orx-split-vec/blob/main/benches/serial_access.rs).*
//!
//! Lastly, we benchmark the case where we access each element of the vector in order starting from the first element to the last. We use the same standard vector as the baseline. For completeness, baseline is compared with `Linear` and `Doubling` strategies; however, `SplitVec` actually uses the same iterator to allow for the serial access for any growth startegy.
//!
//! <img src="https://raw.githubusercontent.com/orxfun/orx-split-vec/main/docs/img/bench_serial_access.PNG" alt="https://raw.githubusercontent.com/orxfun/orx-split-vec/main/docs/img/bench_serial_access.PNG" />
//!
//! The results show that there are minor deviations but no significant difference between the variants.
//!
//! ## F. Relation to the `ImpVec`
//!
//! Providing pinned memory location elements with `PinnedVec` is the first block for building self referential structures; the second building block is the [`ImpVec`](https://crates.io/crates/orx-imp-vec). An `ImpVec` wraps any `PinnedVec` implementation and provides specialized methods built on the pinned element guarantee in order to allow building self referential collections.
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
