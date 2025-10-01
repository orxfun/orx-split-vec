use crate::GrowthWithConstantTimeAccess;
use crate::concurrent_pinned_vec::iter_ptr::IterPtrOfCon;
use crate::{Doubling, Linear};
use std::string::String;
use test_case::test_matrix;

#[test_matrix([
    Doubling,
    Linear::new(4),
])]
fn iter_ptr_default<G>(_: G)
where
    G: GrowthWithConstantTimeAccess,
{
    let iter = IterPtrOfCon::<String, G>::default();
    for _ in iter {}
}
