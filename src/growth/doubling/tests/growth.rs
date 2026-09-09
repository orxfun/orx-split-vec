use crate::growth::doubling::constants::{
    CAPACITIES_LEN, CUMULATIVE_CAPACITIES, FIRST_FRAGMENT_CAPACITY,
};
use crate::{Doubling, Fragment, Growth, GrowthWithConstantTimeAccess, PinnedVec, SplitVec};
use alloc::vec::Vec;

#[test]
fn new_cap() {
    fn new_fra(cap: usize) -> Fragment<usize> {
        Fragment::new_empty(cap)
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
        let mut vec = Vec::with_capacity(4);
        vec.extend(0..4);
        Fragment::new(4, vec)
    }
    fn new_half() -> Fragment<usize> {
        let mut vec = Vec::with_capacity(8);
        for i in 0..4 {
            vec.push(10 + i);
        }
        Fragment::new(8, vec)
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

#[test]
fn get_fragment_and_inner_indices() {
    let growth = Doubling;

    let get = |index| growth.get_fragment_and_inner_indices::<char>(usize::MAX, &[], index);
    let get_none = |index| growth.get_fragment_and_inner_indices::<char>(index, &[], index);

    assert_eq!((0, 0), growth.get_fragment_and_inner_indices_unchecked(0));
    assert_eq!((0, 1), growth.get_fragment_and_inner_indices_unchecked(1));
    assert_eq!((1, 0), growth.get_fragment_and_inner_indices_unchecked(4));
    assert_eq!((1, 5), growth.get_fragment_and_inner_indices_unchecked(9));
    assert_eq!((2, 0), growth.get_fragment_and_inner_indices_unchecked(12));

    assert_eq!(Some((0, 0)), get(0));
    assert_eq!(Some((0, 1)), get(1));
    assert_eq!(Some((1, 0)), get(4));
    assert_eq!(Some((1, 5)), get(9));
    assert_eq!(Some((2, 0)), get(12));

    assert_eq!(None, get_none(0));
    assert_eq!(None, get_none(1));
    assert_eq!(None, get_none(4));
    assert_eq!(None, get_none(9));
    assert_eq!(None, get_none(12));
}

#[test]
fn get_fragment_and_inner_indices_exhaustive() {
    let growth = Doubling;

    let get = |index| growth.get_fragment_and_inner_indices::<char>(usize::MAX, &[], index);
    let get_none = |index| growth.get_fragment_and_inner_indices::<char>(index, &[], index);

    let mut f = 0;
    let mut prev_cumulative_capacity = 0;
    let mut curr_capacity = 4;
    let mut cumulative_capacity = 4;

    for index in 0..51_111 {
        if index == cumulative_capacity {
            prev_cumulative_capacity = cumulative_capacity;
            curr_capacity *= 2;
            cumulative_capacity += curr_capacity;
            f += 1;
        }

        let (f, i) = (f, index - prev_cumulative_capacity);
        assert_eq!(
            (f, i),
            growth.get_fragment_and_inner_indices_unchecked(index)
        );
        assert_eq!(Some((f, i)), get(index));
        assert_eq!(None, get_none(index));
    }
}

#[test]
fn maximum_concurrent_capacity() {
    fn max_cap<T>(vec: &SplitVec<T, Doubling>) -> usize {
        vec.growth()
            .maximum_concurrent_capacity(vec.fragments(), vec.fragments.capacity())
    }

    let mut vec: SplitVec<char, Doubling> = SplitVec::with_doubling_growth();
    assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32);

    let until = max_cap(&vec);
    for _ in 0..until {
        vec.push('x');
        assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32);
    }

    // fragments allocate beyond max_cap
    vec.push('x');
    assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32 + 64 + 128 + 256 + 512);
}

#[test]
fn with_doubling_growth_and_fragments_capacity_normal_growth() {
    let mut vec: SplitVec<char, _> = SplitVec::with_doubling_growth_and_fragments_capacity(1);

    assert_eq!(1, vec.fragments.capacity());

    #[cfg(not(miri))]
    let n = 100_000;
    #[cfg(miri)]
    let n = 44;

    for _ in 0..n {
        vec.push('x');
    }

    #[cfg(not(miri))]
    assert!(vec.fragments.capacity() > 4);
}

#[test]
#[should_panic]
fn with_doubling_growth_and_fragments_capacity_zero() {
    let _: SplitVec<char, _> = SplitVec::with_doubling_growth_and_fragments_capacity(0);
}

#[test]
fn with_doubling_growth_and_fragments_capacity_with_max_fragments_capacity() {
    let vec: SplitVec<char, _> =
        SplitVec::with_doubling_growth_and_fragments_capacity(CAPACITIES_LEN);
    assert_eq!(
        vec.maximum_concurrent_capacity(),
        (1 << (CAPACITIES_LEN + 2)) - FIRST_FRAGMENT_CAPACITY
    );
}

#[test]
#[should_panic]
fn with_doubling_growth_and_fragments_capacity_too_large_fragments_capacity() {
    let _: SplitVec<char, _> =
        SplitVec::with_doubling_growth_and_fragments_capacity(CAPACITIES_LEN + 1);
}

#[test]
fn with_doubling_growth_and_max_concurrent_capacity() {
    let vec: SplitVec<char, _> = SplitVec::with_doubling_growth_and_max_concurrent_capacity();
    assert_eq!(
        vec.maximum_concurrent_capacity(),
        (1 << (CAPACITIES_LEN + 2)) - FIRST_FRAGMENT_CAPACITY
    );
}

#[test]
fn required_fragments_len() {
    let vec: SplitVec<char, Doubling> = SplitVec::with_doubling_growth();
    let num_fragments = |max_cap| {
        vec.growth()
            .required_fragments_len(vec.fragments(), max_cap)
    };

    // 4 - 12 - 28 - 60 - 124
    assert_eq!(num_fragments(0), Ok(0));
    assert_eq!(num_fragments(1), Ok(1));
    assert_eq!(num_fragments(4), Ok(1));
    assert_eq!(num_fragments(5), Ok(2));
    assert_eq!(num_fragments(12), Ok(2));
    assert_eq!(num_fragments(13), Ok(3));
    assert_eq!(num_fragments(36), Ok(4));
    assert_eq!(num_fragments(67), Ok(5));
    assert_eq!(num_fragments(136), Ok(6));
}

#[test]
fn required_fragments_len_at_max() {
    let vec: SplitVec<char, Doubling> = SplitVec::with_doubling_growth();
    let num_fragments = |max_cap| {
        vec.growth()
            .required_fragments_len(vec.fragments(), max_cap)
    };

    let maximum_possible_capacity = *CUMULATIVE_CAPACITIES.last().expect("is not empty");
    #[cfg(target_pointer_width = "32")]
    assert_eq!(num_fragments(maximum_possible_capacity), Ok(29));
    #[cfg(target_pointer_width = "64")]
    assert_eq!(num_fragments(maximum_possible_capacity), Ok(32));
}

#[test]
fn required_fragments_len_more_than_max() {
    let vec: SplitVec<char, Doubling> = SplitVec::with_doubling_growth();
    let num_fragments = |max_cap| {
        vec.growth()
            .required_fragments_len(vec.fragments(), max_cap)
    };

    let more_than_max_possible_capacity = *CUMULATIVE_CAPACITIES.last().expect("is not empty") + 1;
    assert!(num_fragments(more_than_max_possible_capacity).is_err());
}
