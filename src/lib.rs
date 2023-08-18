//! A split vector is a vector represented as a sequence of multiple contagious data fragments
//! which avoids copies while growing and preserves the memory location of its elements.

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

mod default;
mod eq;
mod fragment;
mod from;
mod growth;
mod index;
mod iter;
mod resize_multiple;
mod resize_unit;
mod slice;
mod split_vec_struct;
mod vec;

pub use fragment::fragment_struct::Fragment;
pub use growth::FragmentGrowth;
pub use iter::iterator::SplitVecIterator;
pub use slice::SplitVecSlice;
pub use split_vec_struct::SplitVec;
