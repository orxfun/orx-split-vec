#![doc = include_str!("../README.md")]
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
#![no_std]

#[cfg(test)]
extern crate std;

extern crate alloc;

mod algorithms;
mod common_traits;
mod concurrent_iter;
mod concurrent_pinned_vec;
mod fragment;
mod growth;
mod into_concurrent_pinned_vec;
mod new_split_vec;
mod pinned_vec;
mod pointers;
mod range_helpers;
mod resize_multiple;
mod slice;
mod split_vec;

#[cfg(test)]
pub(crate) mod test;

/// Common relevant traits, structs, enums.
pub mod prelude;

pub use common_traits::iterator::{IntoIter, Iter, IterMut, IterMutRev, IterRev};
pub use concurrent_pinned_vec::ConcurrentSplitVec;
pub use fragment::fragment_struct::Fragment;
pub use fragment::into_fragments::IntoFragments;
pub use growth::par_growth::ParGrowth;
pub use growth::{
    doubling::Doubling,
    growth_trait::{Growth, GrowthWithConstantTimeAccess},
    linear::Linear,
    recursive::Recursive,
};
pub use orx_iterable::{Collection, CollectionMut, Iterable};
pub use orx_pinned_vec::{
    ConcurrentPinnedVec, IntoConcurrentPinnedVec, PinnedVec, PinnedVecGrowthError,
};
pub use orx_pseudo_default::PseudoDefault;
pub use slice::SplitVecSlice;
pub use split_vec::SplitVec;
