use crate::growth::linear::constants::FIXED_CAPACITIES;
use crate::{
    Doubling, Fragment, Growth, GrowthWithConstantTimeAccess, Linear, PinnedVec, SplitVec,
};
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
fn fragment_capacity_linear() {
    let growth = Linear::new(10);

    let capacity = u32::pow(2, 10) as usize;

    for f in 0..100 {
        assert_eq!(growth.fragment_capacity_of(f), capacity);
    }
}

#[test]
fn reserve_for_maximum_concurrent_capacity() {
    let max_capacity = Linear::new(10).maximum_concurrent_capacity_bound::<char>(&[], 0);

    #[cfg(target_pointer_width = "32")]
    assert_eq!(max_capacity, 268_435_456);

    #[cfg(target_pointer_width = "64")]
    assert_eq!(max_capacity, 2_147_483_648);
}

#[test]
fn get_fragment_and_inner_indices() {
    let growth = Linear::new(2);

    let get = |index| growth.get_fragment_and_inner_indices::<char>(usize::MAX, &[], index);
    let get_none = |index| growth.get_fragment_and_inner_indices::<char>(index, &[], index);

    assert_eq!((0, 0), growth.get_fragment_and_inner_indices_unchecked(0));
    assert_eq!((0, 1), growth.get_fragment_and_inner_indices_unchecked(1));
    assert_eq!((1, 0), growth.get_fragment_and_inner_indices_unchecked(4));
    assert_eq!((2, 1), growth.get_fragment_and_inner_indices_unchecked(9));
    assert_eq!((4, 0), growth.get_fragment_and_inner_indices_unchecked(16));

    assert_eq!(Some((0, 0)), get(0));
    assert_eq!(Some((0, 1)), get(1));
    assert_eq!(Some((1, 0)), get(4));
    assert_eq!(Some((2, 1)), get(9));
    assert_eq!(Some((4, 0)), get(16));

    assert_eq!(None, get_none(0));
    assert_eq!(None, get_none(1));
    assert_eq!(None, get_none(4));
    assert_eq!(None, get_none(9));
    assert_eq!(None, get_none(16));
}

#[test]
fn get_fragment_and_inner_indices_exhaustive() {
    let growth = Linear::new(5);

    let get = |index| growth.get_fragment_and_inner_indices::<char>(usize::MAX, &[], index);
    let get_none = |index| growth.get_fragment_and_inner_indices::<char>(index, &[], index);

    let curr_capacity = 32;

    let mut f = 0;
    let mut prev_cumulative_capacity = 0;
    let mut cumulative_capacity = curr_capacity;

    for index in 0..51_111 {
        if index == cumulative_capacity {
            prev_cumulative_capacity = cumulative_capacity;
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
    fn max_cap<T>(vec: &SplitVec<T, Linear>) -> usize {
        vec.growth()
            .maximum_concurrent_capacity(vec.fragments(), vec.fragments.capacity())
    }

    let mut vec: SplitVec<char, Linear> = SplitVec::with_linear_growth(5);
    assert_eq!(max_cap(&vec), 4 * 2usize.pow(5));

    let until = max_cap(&vec);
    for _ in 0..until {
        vec.push('x');
        assert_eq!(max_cap(&vec), 4 * 2usize.pow(5));
    }

    // fragments allocate beyond max_cap
    vec.push('x');
    assert_eq!(max_cap(&vec), 8 * 2usize.pow(5));
}

#[test]
fn with_linear_growth_and_fragments_capacity_normal_growth() {
    let mut vec: SplitVec<char, _> = SplitVec::with_linear_growth_and_fragments_capacity(10, 1);

    assert_eq!(1, vec.fragments.capacity());

    #[cfg(not(miri))]
    let n = 100_000;
    #[cfg(miri)]
    let n = 55;

    for _ in 0..n {
        vec.push('x');
    }

    #[cfg(not(miri))]
    assert!(vec.fragments.capacity() > 4);
}

#[test]
#[should_panic]
fn with_linear_growth_and_fragments_capacity_zero() {
    let _: SplitVec<char, _> = SplitVec::with_linear_growth_and_fragments_capacity(10, 0);
}

#[test]
fn with_linear_growth_with_max_fragment_capacity() {
    let exponent = FIXED_CAPACITIES.len() - 1;
    let _: SplitVec<char, _> = SplitVec::with_linear_growth(exponent);
}

#[test]
#[should_panic]
fn with_linear_growth_with_too_large_fragment_capacity() {
    let exponent = FIXED_CAPACITIES.len();
    let _: SplitVec<char, _> = SplitVec::with_linear_growth(exponent);
}

#[test]
fn required_fragments_len() {
    let vec: SplitVec<char, Linear> = SplitVec::with_linear_growth(5);
    let num_fragments = |max_cap| {
        vec.growth()
            .required_fragments_len(vec.fragments(), max_cap)
    };

    assert_eq!(num_fragments(0), Ok(0));
    assert_eq!(num_fragments(1), Ok(1));
    assert_eq!(num_fragments(2), Ok(1));
    assert_eq!(num_fragments(32), Ok(1));
    assert_eq!(num_fragments(33), Ok(2));
    assert_eq!(num_fragments(32 * 7), Ok(7));
    assert_eq!(num_fragments(32 * 7 + 1), Ok(8));
}
