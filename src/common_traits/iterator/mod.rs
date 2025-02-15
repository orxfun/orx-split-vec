mod concurrent_iterators;
mod eq;
mod flattened_iter_of_slices;
mod from_iter;
mod into_iter;
mod iter;
mod iter_mut;
mod iter_mut_rev;
mod iter_of_slices;
mod iter_of_slices_of_con;
mod iter_ptr;
mod iter_ptr_bwd;
mod iter_rev;
mod reductions;

#[cfg(test)]
mod tests;

pub use into_iter::IntoIter;
pub use iter::Iter;
pub use iter_mut::IterMut;
pub use iter_mut_rev::IterMutRev;
pub use iter_of_slices::{IterOfSlices, SliceBorrowAsMut, SliceBorrowAsRef};
pub use iter_of_slices_of_con::IterOfSlicesOfCon;
pub use iter_ptr::IterPtr;
pub use iter_ptr_bwd::IterPtrBackward;
pub use iter_rev::IterRev;
