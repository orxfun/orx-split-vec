use crate::{
    Doubling, Fragment, Growth, GrowthWithConstantTimeAccess,
    growth::doubling::constants::CAPACITIES_LEN,
};
use alloc::vec::Vec;

#[test]
fn new_cap() {
    fn new_fra(cap: usize) -> Fragment<usize> {
        Vec::<usize>::with_capacity(cap).into()
    }

    let growth = Doubling;
    assert_eq!(4, growth.new_fragment_capacity(&[new_fra(2)]));
    assert_eq!(12, growth.new_fragment_capacity(&[new_fra(3), new_fra(6)]));
    assert_eq!(
        56,
        growth.new_fragment_capacity(&[new_fra(7), new_fra(14), new_fra(28)])
    );
}

#[test]
fn indices_panics_when_fragments_is_empty() {
    assert_eq!(
        None,
        <Doubling as Growth>::get_fragment_and_inner_indices::<usize>(&Doubling, 0, &[], 0)
    );
}

#[test]
fn indices() {
    fn new_full() -> Fragment<usize> {
        (0..4).collect::<Vec<_>>().into()
    }
    fn new_half() -> Fragment<usize> {
        let mut vec = Vec::with_capacity(8);
        for i in 0..4 {
            vec.push(10 + i);
        }
        vec.into()
    }

    let growth = Doubling;

    for i in 0..4 {
        assert_eq!(
            Some((0, i)),
            growth.get_fragment_and_inner_indices(4, &[new_full()], i)
        );
    }
    assert_eq!(
        None,
        growth.get_fragment_and_inner_indices(4, &[new_full()], 4)
    );

    for i in 0..4 {
        assert_eq!(
            Some((0, i)),
            growth.get_fragment_and_inner_indices(8, &[new_full(), new_half()], i)
        );
    }
    for i in 4..8 {
        assert_eq!(
            Some((1, i - 4)),
            growth.get_fragment_and_inner_indices(8, &[new_full(), new_half()], i)
        );
    }
    assert_eq!(
        None,
        growth.get_fragment_and_inner_indices(8, &[new_full(), new_half()], 12)
    );
}

#[test]
fn fragment_capacity_doubling() {
    let growth = Doubling;

    let mut capacity = 4;

    for f in 0..CAPACITIES_LEN {
        assert_eq!(growth.fragment_capacity_of(f), capacity);
        capacity *= 2;
    }
}

#[test]
fn reserve_for_maximum_concurrent_capacity() {
    let max_capacity = Doubling.maximum_concurrent_capacity_bound::<char>(&[], 0);

    #[cfg(target_pointer_width = "32")]
    assert_eq!(max_capacity, 2_147_483_644);

    #[cfg(target_pointer_width = "64")]
    assert_eq!(max_capacity, 17_179_869_180);
}
