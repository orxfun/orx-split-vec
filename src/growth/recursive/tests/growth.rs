use crate::growth::doubling::constants::{CAPACITIES_LEN, FIRST_FRAGMENT_CAPACITY};
use crate::{Fragment, Growth, Recursive, SplitVec};
use alloc::vec::Vec;
use orx_pinned_vec::PinnedVec;

#[test]
fn new_cap() {
    fn new_fra(cap: usize) -> Fragment<usize> {
        Fragment::new_empty(cap)
    }

    let growth = Recursive;
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
        <Recursive as Growth>::get_fragment_and_inner_indices::<usize>(&Recursive, 0, &[], 0)
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

    let growth = Recursive;

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
fn reserve_for_maximum_concurrent_capacity() {
    let max_capacity = Recursive.maximum_concurrent_capacity_bound::<char>(&[], 0);

    #[cfg(target_pointer_width = "32")]
    assert_eq!(max_capacity, 2_147_483_644);

    #[cfg(target_pointer_width = "64")]
    assert_eq!(max_capacity, 17_179_869_180);
}

#[test]
fn get_fragment_and_inner_indices() {
    let growth = Recursive;

    let vecs = alloc::vec![
        alloc::vec![0, 1, 2, 3],
        alloc::vec![4, 5],
        alloc::vec![6, 7, 8],
        alloc::vec![9],
        alloc::vec![10, 11, 12, 13, 14],
    ];
    let mut fragments: Vec<Fragment<_>> = vecs
        .clone()
        .into_iter()
        .map(|x| Fragment::new(x.capacity(), x))
        .collect();
    let len = fragments.iter().map(|x| x.len()).sum();

    let mut index = 0;
    for (f, vec) in vecs.iter().enumerate() {
        for (i, _) in vec.iter().enumerate() {
            let maybe_fi = growth.get_fragment_and_inner_indices(len, &fragments, index);
            assert_eq!(maybe_fi, Some((f, i)));

            let ptr = growth.get_ptr_mut(&mut fragments, index).expect("is-some");
            assert_eq!(unsafe { *ptr }, index);

            unsafe { *ptr = 10 * index };
            assert_eq!(unsafe { *ptr }, 10 * index);

            index += 1;
        }
    }
}

#[test]
fn get_fragment_and_inner_indices_exhaustive() {
    let growth = Recursive;

    let mut fragments: Vec<Fragment<_>> = alloc::vec![];

    #[cfg(not(miri))]
    let lengths = [30, 1, 7, 3, 79, 147, 530];
    #[cfg(miri)]
    let lengths = [1, 7, 3, 30];

    let mut index = 0;
    for _ in 0..10 {
        for &len in &lengths {
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(index);
                index += 1;
            }
            let fragment = Fragment::new(len, vec);
            fragments.push(fragment);
        }
    }

    let total_len = fragments.iter().map(|x| x.len()).sum();

    let mut index = 0;
    let mut f = 0;
    for _ in 0..10 {
        for &len in &lengths {
            for i in 0..len {
                let maybe_fi = growth.get_fragment_and_inner_indices(total_len, &fragments, index);

                assert_eq!(maybe_fi, Some((f, i)));

                let ptr = growth.get_ptr_mut(&mut fragments, index).expect("is-some");
                assert_eq!(unsafe { *ptr }, index);

                unsafe { *ptr = 10 * index };
                assert_eq!(unsafe { *ptr }, 10 * index);

                index += 1;
            }
            f += 1;
        }
    }
}

#[test]
fn maximum_concurrent_capacity() {
    fn max_cap<T>(vec: &SplitVec<T, Recursive>) -> usize {
        vec.growth()
            .maximum_concurrent_capacity(vec.fragments(), vec.fragments.capacity())
    }

    let mut vec: SplitVec<char, Recursive> = SplitVec::with_recursive_growth();
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
fn maximum_concurrent_capacity_when_appended() {
    fn max_cap<T>(vec: &SplitVec<T, Recursive>) -> usize {
        vec.growth()
            .maximum_concurrent_capacity(vec.fragments(), vec.fragments.capacity())
    }

    let mut vec: SplitVec<char, Recursive> = SplitVec::with_recursive_growth();
    assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32);

    vec.append(alloc::vec!['x'; 10]);
    assert_eq!(vec.fragments().len(), 2);
    assert_eq!(vec.fragments()[1].capacity(), 10);
    assert_eq!(vec.fragments()[1].len(), 10);

    assert_eq!(max_cap(&vec), 4 + 10 + 20 + 40);
}

#[test]
fn with_recursive_growth_and_fragments_capacity_normal_growth() {
    let mut vec: SplitVec<char, _> = SplitVec::with_recursive_growth_and_fragments_capacity(1);

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
fn with_recursive_growth_and_fragments_capacity_zero() {
    let _: SplitVec<char, _> = SplitVec::with_recursive_growth_and_fragments_capacity(0);
}

#[test]
#[should_panic]
fn with_recursive_growth_and_fragments_capacity_too_large_fragments_capacity() {
    let vec: SplitVec<char, _> = SplitVec::with_recursive_growth_and_fragments_capacity(1000);
    assert_eq!(
        vec.maximum_concurrent_capacity(),
        (1 << (CAPACITIES_LEN + 2)) - FIRST_FRAGMENT_CAPACITY
    );
}

#[test]
fn with_recursive_growth_and_max_concurrent_capacity() {
    let vec: SplitVec<char, _> = SplitVec::with_recursive_growth_and_max_concurrent_capacity();
    assert_eq!(
        vec.maximum_concurrent_capacity(),
        (1 << (CAPACITIES_LEN + 2)) - FIRST_FRAGMENT_CAPACITY
    );
}

#[test]
fn required_fragments_len() {
    let vec: SplitVec<char, Recursive> = SplitVec::with_recursive_growth();
    let num_fragments = |max_cap| {
        vec.growth()
            .required_fragments_len(vec.fragments(), max_cap)
    };

    // 4 - 12 - 28 - 60 - 124
    assert_eq!(num_fragments(0), Ok(1));
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
fn required_fragments_len_when_appended() {
    let mut vec: SplitVec<char, Recursive> = SplitVec::with_recursive_growth();
    for _ in 0..4 {
        vec.push('x')
    }
    vec.append(alloc::vec!['x'; 10]);

    let num_fragments = |max_cap| {
        vec.growth()
            .required_fragments_len(vec.fragments(), max_cap)
    };

    // 4 - 10 - 20 - 40 - 80
    // 4 - 14 - 34 - 74 - 154
    assert_eq!(num_fragments(0), Ok(2));
    assert_eq!(num_fragments(1), Ok(2));
    assert_eq!(num_fragments(14), Ok(2));
    assert_eq!(num_fragments(15), Ok(3));
    assert_eq!(num_fragments(21), Ok(3));
    assert_eq!(num_fragments(34), Ok(3));
    assert_eq!(num_fragments(35), Ok(4));
    assert_eq!(num_fragments(74), Ok(4));
    assert_eq!(num_fragments(75), Ok(5));
    assert_eq!(num_fragments(154), Ok(5));
    assert_eq!(num_fragments(155), Ok(6));
}
