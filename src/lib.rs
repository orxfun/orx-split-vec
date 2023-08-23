//! A split vector is a vector represented as a sequence of multiple contagious data fragments which:
//! * preserves the memory location of its elements, and
//! * avoids memory copies while growing.
//!
//! # Pinned elements
//! ```
//! use orx_split_vec::SplitVec;
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
//! for (i, elem) in vec.into_iter().enumerate() {
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
//! # Flexible growth strategies without copies
//!
//! Growth of a split vector happens by allocating a new contagious memory
//! while keeping already written elements in place.
//!
//! This allows to avoid memory copies; however, leads to worse cache locality and
//! slower access through indices.
//!
//! ```
//! use orx_split_vec::{Fragment, SplitVec, SplitVecGrowth};
//! use std::rc::Rc;
//!
//! fn custom_growth_fun<T>(fragments: &[Fragment<T>]) -> usize {
//!     if fragments.len() < 4 {
//!         4
//!     } else {
//!         8
//!     }
//! }
//! fn get_fragment_capacities<T, G: SplitVecGrowth<T>>(vec: &SplitVec<T, G>) -> Vec<usize> {
//!     vec.fragments().iter().map(|f| f.capacity()).collect()
//! }
//! fn get_fragment_lengths<T, G: SplitVecGrowth<T>>(vec: &SplitVec<T, G>) -> Vec<usize> {
//!     vec.fragments().iter().map(|f| f.len()).collect()
//! }
//!
//! // let's create 4 vectors with different growth strategies
//! let mut vec_lin = SplitVec::with_linear_growth(10);
//! let mut vec_dbl = SplitVec::with_doubling_growth(4);
//! let mut vec_exp = SplitVec::with_exponential_growth(4, 1.5);
//! let mut vec_custom = SplitVec::with_custom_growth_function(Rc::new(custom_growth_fun));
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
//! assert_eq!(
//!     vec![10, 10, 10, 10],
//!     get_fragment_capacities(&vec_lin)
//! );
//! assert_eq!(
//!     vec![10, 10, 10, 5],
//!     get_fragment_lengths(&vec_lin)
//! );
//!
//! // # doubling: fragment capacities keep doubling
//! assert_eq!(
//!     vec![4, 8, 16, 32],
//!     get_fragment_capacities(&vec_dbl)
//! );
//! assert_eq!(
//!     vec![4, 8, 16, 7],
//!     get_fragment_lengths(&vec_dbl)
//! );
//!
//! // # exponential: fragment capacities grow exponentially with given growth factor
//! assert_eq!(
//!     vec![4, 6, 9, 13, 19],
//!     get_fragment_capacities(&vec_exp)
//! );
//! assert_eq!(
//!     vec![4, 6, 9, 13, 3],
//!     get_fragment_lengths(&vec_exp)
//! );
//!
//! // # custom: pretty much any growth strategy
//! assert_eq!(
//!     vec![4, 4, 4, 4, 8, 8, 8],
//!     get_fragment_capacities(&vec_custom)
//! );
//! assert_eq!(
//!     vec![4, 4, 4, 4, 8, 8, 3],
//!     get_fragment_lengths(&vec_custom)
//! );
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

mod eq;
mod fragment;
mod growth;
mod index;
mod iter;
mod new_split_vec;
mod resize_multiple;
mod resize_unit;
mod slice;
mod split_vec;
#[cfg(test)]
pub(crate) mod test;
mod vec;

pub use fragment::fragment_struct::Fragment;
pub use growth::{
    custom::CustomGrowth, doubling::DoublingGrowth, exponential::ExponentialGrowth,
    fixed::FixedCapacity, growth_trait::SplitVecGrowth, linear::LinearGrowth,
};
pub use iter::iterator::SplitVecIterator;
pub use slice::SplitVecSlice;
pub use split_vec::SplitVec;
